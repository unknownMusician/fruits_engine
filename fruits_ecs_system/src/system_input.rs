use std::sync::RwLock;

use fruits_ecs_data::WorldData;
use fruits_ecs_system_resource::SystemStatesHolder;

#[derive(Copy, Clone)]
pub struct SystemInput<'a> {
    pub world_data: &'a RwLock<WorldData>,
    pub system_data: &'a SystemStatesHolder,
}
