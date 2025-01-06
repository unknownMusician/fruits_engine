use std::sync::RwLockReadGuard;

use fruits_ecs_component::Entity;
use fruits_ecs_data::WorldData;
use fruits_ecs_data_usage::DataUsage;
use fruits_ecs_system::{SystemInput, SystemParam};

pub struct EntitiesInfo<'d> {
    world: RwLockReadGuard<'d, WorldData>,
}

impl<'d> EntitiesInfo<'d> {
    pub fn exists(&self, entity: Entity) -> bool {
        self.world.entities_components().contains_entity(entity)
    }
}

unsafe impl<'b> SystemParam for EntitiesInfo<'b> {
    type Item<'d> = EntitiesInfo<'d>;

    fn fill_data_usage(usage: &mut DataUsage) {
        usage.add_all_mut();
    }

    fn new<'d>(input: SystemInput<'d>) -> Option<Self::Item<'d>> {
        let guard = input.world_data.try_read().ok()?;

        Some(EntitiesInfo {
            world: guard,
        })
    }
}
