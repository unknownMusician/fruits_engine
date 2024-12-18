use std::{any::TypeId, collections::{BTreeMap, HashMap, HashSet}, sync::Arc};

use crate::order_graph::OrderGraph;

use fruits_ecs_data_usage::*;
use fruits_ecs_system::System;

pub struct SystemInfo {
    pub type_id: TypeId,
    pub system: Arc<dyn System>,
}

pub fn create_ordering_graph(ordered_systems: &[SystemInfo], explicit_ordering: &HashSet<(TypeId, TypeId)>) -> OrderGraph {
    let system_index_by_type = ordered_systems.iter().enumerate().map(|(i, s)| (s.type_id, i)).collect::<HashMap<_, _>>();

    let mut system_by_data_readonly = HashMap::<TypeId, HashSet<usize>>::new();
    let mut system_by_data_mutable = HashMap::<TypeId, HashSet<usize>>::new();
    let mut systems_global_mutable = HashSet::<usize>::new();

    let mut analyzed_systems = HashSet::<usize>::new();

    let mut directions = std::iter::repeat_with(|| HashSet::<usize>::new()).take(ordered_systems.len()).collect::<Box<_>>();

    for (previous_id, next_id) in explicit_ordering.iter() {
        let Some(&previous_index) = system_index_by_type.get(previous_id) else {
            continue;
        };
        
        let Some(&next_index) = system_index_by_type.get(next_id) else {
            continue;
        };

        directions[previous_index].insert(next_index);
    }

    for (system_index, system) in ordered_systems.iter().enumerate() {
        let mut data_usage = DataUsage::new();

        system.system.fill_data_usage(&mut data_usage);

        match data_usage {
            DataUsage::PerType(per_type_usage) => {
                for (type_id, is_mutable) in per_type_usage.values().iter() {
                    if *is_mutable {
                        for &other_readonly_system_index in system_by_data_readonly.get(type_id).iter().map(|m| m.iter()).flatten() {
                            directions[other_readonly_system_index].insert(system_index);
                        }
                        for &other_mutable_system_index in system_by_data_mutable.get(type_id).iter().map(|m| m.iter()).flatten() {
                            directions[other_mutable_system_index].insert(system_index);
                        }
        
                        system_by_data_mutable.entry(*type_id).or_default().insert(system_index);
                    } else {
                        for &other_mutable_system_index in system_by_data_mutable.get(type_id).iter().map(|m| m.iter()).flatten() {
                            directions[other_mutable_system_index].insert(system_index);
                        }
        
                        system_by_data_readonly.entry(*type_id).or_default().insert(system_index);
                    }
                }

                for &other_global_mutable_system_index in systems_global_mutable.iter() {
                    directions[other_global_mutable_system_index].insert(system_index);
                }
            },
            DataUsage::GlobalMutable => {
                for &other_system_index in analyzed_systems.iter() {
                    directions[other_system_index].insert(system_index);
                }

                systems_global_mutable.insert(system_index);
            }
        };

        analyzed_systems.insert(system_index);
    }

    let directions = directions.into_vec().into_iter().map(|v| v.into_iter().collect::<Box<_>>()).collect::<Box<_>>();

    OrderGraph::new(directions).unwrap()
}

pub fn sort_systems_by_order(systems: &HashMap<TypeId, Arc<dyn System>>, systems_ordering: &HashSet<(TypeId, TypeId)>) -> Box<[SystemInfo]> {
    let order_by_type = index_ordering(systems_ordering);

    let mut system_by_order = BTreeMap::new();
    let mut unordered = Vec::new();

    for (system_type, system) in systems.iter() {
        let system_info = SystemInfo {
            type_id: *system_type,
            system: Arc::clone(system),
        };

        if let Some(index) = order_by_type.get(&system_type) {
            system_by_order.insert(index, system_info);
        } else {
            unordered.push(system_info);
        }
    }

    system_by_order.into_values().chain(unordered.into_iter()).collect()
}

fn index_ordering(ordering: &HashSet<(TypeId, TypeId)>) -> HashMap<TypeId, usize> {
    let mut max_to_min = HashMap::<TypeId, Vec<TypeId>>::new();

    for (min, max) in ordering.iter()
    {
        max_to_min.entry(*max).or_default().push(*min);
    }
    
    let mut types_set = HashSet::<TypeId>::new();
    let mut ordered_types = Vec::<TypeId>::new();

    while max_to_min.len() != 0
    {
        let (min, max) = most_min(&max_to_min);

        if types_set.insert(min)
        {
            ordered_types.push(min);
        }

        if let Some(mins) = max_to_min.get_mut(&max)
        {
            mins.remove(mins.iter().position(|m| *m == min).unwrap());
            
            if mins.len() == 0
            {
                if types_set.insert(max)
                {
                    ordered_types.push(max);
                }

                max_to_min.remove(&max);
            }
        }
    }

    let mut orders = HashMap::<TypeId, usize>::new();

    for (index, type_id) in ordered_types.into_iter().enumerate() {
        orders.insert(type_id, index);
    }
    
    orders
}

fn most_min(max_to_min: &HashMap<TypeId, Vec<TypeId>>) -> (TypeId, TypeId)
{
    let mut visited = HashSet::<TypeId>::new();
    
    let (mut max, mut mins) = max_to_min.iter().next().unwrap();
    
    while visited.insert(*max)
    {
        let min = mins.iter().next().unwrap();

        let Some(new_mins) = max_to_min.get(min) else {
            return (*min, *max);
        };

        mins = new_mins;
        max = min;
    }

    panic!("The orderer contains circular dependencies. Cycle contains {} elements.", visited.len());
}