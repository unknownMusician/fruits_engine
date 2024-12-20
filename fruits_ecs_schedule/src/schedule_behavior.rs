use std::{any::{Any, TypeId}, collections::{HashMap, HashSet}, sync::{Arc, Mutex, RwLock}};

use fruits_ecs_data::WorldData;
use fruits_ecs_system::{SystemInput, SystemWithMarker};
use fruits_utils::thread_pool::ThreadPool;
use fruits_ecs_system::System;
use fruits_ecs_system_resource::SystemResourcesHolder;

use crate::order_graph::OrderGraph;

use super::system_order;

pub struct ScheduleBehavior {
    systems: Arc<[Arc<dyn System>]>,
    system_datas: Arc<[Mutex<SystemResourcesHolder>]>,
    execution_graph: Arc<OrderGraph>,
    thread_pool: ThreadPool,
}

impl ScheduleBehavior {
    pub fn new(systems: Arc<[Arc<dyn System>]>, execution_graph: Arc<OrderGraph>) -> Self {
        Self {
            system_datas: systems.iter().map(|_| Mutex::new(SystemResourcesHolder::new())).collect::<Arc<_>>(),
            systems,
            execution_graph,
            thread_pool: ThreadPool::new(Self::non_main_threads_count())
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

                //

                let data = Arc::clone(data);
                let iter = Arc::clone(&iter);
                let systems = Arc::clone(&self.systems);
                let system_datas = Arc::clone(&self.system_datas);

                let job = move || {
                    let system = &systems[system_index];
                    let system_data = &system_datas[system_index];

                    let input = SystemInput {
                        world_data: &*data,
                        system_data: &mut *system_data.try_lock().ok().unwrap(),
                    };
    
                    system.execute(input);
                    
                    {
                        iter.lock().unwrap().end(system_index);
                    }
                };

                //

                self.thread_pool.push_job(Box::new(job));
            }
        }
    }
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

    pub fn add_system<M: 'static>(&mut self, system: impl SystemWithMarker<M> + Any) -> bool {
        self.systems.insert(system.type_id(), Arc::from(system.into_system_generic())).is_none()
    }

    pub fn order_systems<M0: 'static, M1: 'static>(
        &mut self,
        previous_system: impl SystemWithMarker<M0> + Any,
        next_system: impl SystemWithMarker<M1> + Any,
    ) {
        self.systems_ordering.insert((previous_system.type_id(), next_system.type_id()));
    }

    pub fn build(self) -> ScheduleBehavior {
        let systems = system_order::sort_systems_by_order(&self.systems, &self.systems_ordering);

        let execution_graph = system_order::create_ordering_graph(&systems, &self.systems_ordering);

        let systems = systems.iter().map(|s| Arc::clone(&s.system)).collect::<Arc<_>>();

        ScheduleBehavior::new(systems, Arc::new(execution_graph))
    }
}
