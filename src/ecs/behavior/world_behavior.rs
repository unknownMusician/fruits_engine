use std::{any::{Any, TypeId}, collections::{BTreeMap, HashMap, HashSet}, sync::{Arc, Mutex, RwLock}};

use crate::{ecs::data::world_data::WorldData, thread_pool::ThreadPool};

use super::{graph::ExecutionGraph, system::{System, SystemWithMarker}};

pub struct SingleWorldSharedDataUsage {
    pub data_type: TypeId,
    pub is_mutable: bool,
}

impl SingleWorldSharedDataUsage {
    pub fn new(data_type: TypeId, is_mutable: bool) -> Self {
        Self {
            data_type,
            is_mutable,
        }
    }
    pub fn new_mutable(type_id: TypeId) -> Self {
        Self::new(type_id, true)
    }
    pub fn new_readonly(type_id: TypeId) -> Self {
        Self::new(type_id, false)
    }
}

pub struct WorldSharedPerTypeDataUsage {
    is_mutable: HashMap<TypeId, bool>
}

impl WorldSharedPerTypeDataUsage {
    pub fn new() -> Self {
        Self {
            is_mutable: HashMap::new(),
        }
    }

    pub fn add(&mut self, usage: SingleWorldSharedDataUsage) {
        *self.is_mutable.entry(usage.data_type).or_default() |= usage.is_mutable;
    }

    pub fn values(&self) -> &HashMap<TypeId, bool> {
        &self.is_mutable
    }
}

pub enum WorldSharedDataUsage {
    PerType(WorldSharedPerTypeDataUsage),
    // todo: global immutable?
    GlobalMutable,
}

impl WorldSharedDataUsage {
    pub fn new() -> Self {
        WorldSharedDataUsage::PerType(WorldSharedPerTypeDataUsage::new())
    }

    pub fn add(&mut self, usage: SingleWorldSharedDataUsage) {
        let WorldSharedDataUsage::PerType(per_type) = self else {
            return;
        };

        per_type.add(usage);
    }

    pub fn add_all_mut(&mut self) {
        *self = WorldSharedDataUsage::GlobalMutable;
    }
}

struct SystemIterationJob {
    iter: Arc<Mutex<super::graph::ExecutionGraphIterator>>,
    systems: Arc<[Arc<dyn System>]>,
    data: Arc<RwLock<WorldData>>,
    system_index: usize
}

#[derive(Clone, Copy)]
pub enum Schedule {
    Start = 0,
    Update = 1,
}

impl Schedule {
    pub const fn count() -> usize { 2 }
    pub const fn index(&self) -> usize { *self as usize }
}

pub struct WorldBehaviorBuilder {
    schedule_behaviors: [ScheduleBehaviorBuilder; Schedule::count()],
}

impl WorldBehaviorBuilder {
    pub fn new() -> Self {
        Self {
            schedule_behaviors: core::array::from_fn::<_, { Schedule::count() }, _>(|_| ScheduleBehaviorBuilder::new()),
        }
    }

    pub fn get_mut(&mut self, schedule: Schedule) -> &mut ScheduleBehaviorBuilder {
        &mut self.schedule_behaviors[schedule.index()]
    }

    pub fn build(self) -> WorldBehavior {
        WorldBehavior {
            schedule_behaviors: self.schedule_behaviors.map(|b| b.build()),
        }
    }
}

pub struct WorldBehavior {
    schedule_behaviors: [ScheduleBehavior; Schedule::count()],
}

impl WorldBehavior {
    pub fn get(&self, schedule: Schedule) -> &ScheduleBehavior {
        &self.schedule_behaviors[schedule.index()]
    }
}

pub struct ScheduleBehavior {
    systems: Arc<[Arc<dyn System>]>,
    execution_graph: Arc<ExecutionGraph>,
    thread_pool: ThreadPool<SystemIterationJob>,
}

impl ScheduleBehavior {
    pub fn new(systems: Arc<[Arc<dyn System>]>, execution_graph: Arc<ExecutionGraph>) -> Self {
        Self {
            systems,
            execution_graph,
            thread_pool: ThreadPool::new(Self::non_main_threads_count(), move |j: SystemIterationJob| {
                let system = &j.systems[j.system_index];
                system.borrow_and_execute(&*j.data);
                
                {
                    j.iter.lock().unwrap().end(j.system_index);
                }
            })
        }
    }

    fn non_main_threads_count() -> usize {
        match std::thread::available_parallelism() {
            Ok(count) => (count.get() - 1).max(1),
            Err(_) => 3,
        }
    }

    pub fn execute_iteration(&self, data: &Arc<RwLock<WorldData>>) {
        let iter = Arc::new(Mutex::new(self.execution_graph.iter()));

        loop {
            let system_index = {
                let mut iter = iter.lock().unwrap();
                
                if iter.all_ended() {
                    break;
                }

                iter.start_next()
            };

            if let Some(system_index) = system_index {
                self.thread_pool.push_job(SystemIterationJob {
                    data: Arc::clone(data),
                    iter: Arc::clone(&iter),
                    systems: Arc::clone(&self.systems),
                    system_index,
                });
            }
        }
    }
}

struct SystemInfo {
    pub type_id: TypeId,
    pub system: Arc<dyn System>,
}

pub struct ScheduleBehaviorBuilder {
    systems: HashMap<TypeId, Arc<dyn System>>,
    systems_ordering: HashSet<(TypeId, TypeId)>
}

impl ScheduleBehaviorBuilder {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
            systems_ordering: HashSet::new(),
        }
    }

    pub fn add_system<M>(&mut self, system: impl SystemWithMarker<M> + Any) -> bool {
        self.systems.insert(system.type_id(), Arc::from(system.into_system_generic())).is_none()
    }

    pub fn order_systems<M0, M1>(&mut self, previous_system: impl SystemWithMarker<M0> + Any, next_system: impl SystemWithMarker<M1> + Any) {
        self.systems_ordering.insert((previous_system.type_id(), next_system.type_id()));
    }

    pub fn build(self) -> ScheduleBehavior {
        let systems = sort_systems_by_order(&self.systems, &self.systems_ordering);

        let execution_graph = create_execution_graph(&systems, &self.systems_ordering);

        let systems = systems.iter().map(|s| Arc::clone(&s.system)).collect::<Arc<_>>();

        ScheduleBehavior::new(systems, Arc::new(execution_graph))
    }
}

fn create_execution_graph(ordered_systems: &[SystemInfo], explicit_ordering: &HashSet<(TypeId, TypeId)>) -> ExecutionGraph {
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
        let mut data_usage = WorldSharedDataUsage::new();

        system.system.fill_data_usage(&mut data_usage);

        match data_usage {
            WorldSharedDataUsage::PerType(per_type_usage) => {
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
            WorldSharedDataUsage::GlobalMutable => {
                for &other_system_index in analyzed_systems.iter() {
                    directions[other_system_index].insert(system_index);
                }

                systems_global_mutable.insert(system_index);
            }
        };

        analyzed_systems.insert(system_index);
    }

    let directions = directions.into_vec().into_iter().map(|v| v.into_iter().collect::<Box<_>>()).collect::<Box<_>>();

    ExecutionGraph::new(directions).unwrap()
}

fn sort_systems_by_order(systems: &HashMap<TypeId, Arc<dyn System>>, systems_ordering: &HashSet<(TypeId, TypeId)>) -> Box<[SystemInfo]> {
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