use fruits_ecs_schedule::WorldBehaviorBuilder;

use fruits_ecs_data::WorldData;

use crate::world::World;

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
