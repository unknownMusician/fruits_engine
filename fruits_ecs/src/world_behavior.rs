use std::sync::{Arc, Mutex, RwLock};

use crate::{data::world_data::WorldData, order_graph::OrderGraphIterator, system::System, system_data::SystemStatesHolder};

use super::schedule_behavior::{ScheduleBehavior, ScheduleBehaviorBuilder};

struct SystemIterationJob {
    iter: Arc<Mutex<OrderGraphIterator>>,
    systems: Arc<[Arc<dyn System>]>,
    system_datas: Arc<[Mutex<SystemStatesHolder>]>,
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
