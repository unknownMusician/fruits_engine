use std::sync::{Arc, RwLock};

use fruits_ecs_data::WorldData;
use fruits_ecs_schedule::{Schedule, WorldBehavior};

pub struct World {
    data: Arc<RwLock<WorldData>>,
    behavior: WorldBehavior,
}

impl World {
    pub fn new(data: WorldData, behavior: WorldBehavior) -> Self {
        Self {
            data: Arc::new(RwLock::new(data)),
            behavior,
        }
    }

    pub fn execute_iteration(&self, schedule: Schedule) {
        self.behavior.get(schedule).execute_iteration(&self.data);
    }
}
