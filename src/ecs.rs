use std::sync::{Arc, RwLock};

use self::{behavior::{Schedule, WorldBehavior, WorldBehaviorBuilder}, data::world_data::WorldData};

pub mod behavior;
pub mod data;

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

pub struct WorldBuilder {
    data: WorldData,
    behavior: WorldBehaviorBuilder,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self {
            behavior: WorldBehaviorBuilder::new(),
            data: WorldData::new(),
        }
    }

    pub fn behavior(&self) -> &WorldBehaviorBuilder {
        &self.behavior
    }

    pub fn behavior_mut(&mut self) -> &mut WorldBehaviorBuilder {
        &mut self.behavior
    }

    pub fn data(&self) -> &WorldData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut WorldData {
        &mut self.data
    }

    pub fn build(self) -> World {
        World::new(self.data, self.behavior.build())
    }
}