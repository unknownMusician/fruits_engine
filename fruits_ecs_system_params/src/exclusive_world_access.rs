use std::{ops::{Deref, DerefMut}, sync::RwLockWriteGuard};

use fruits_ecs_data::WorldData;
use fruits_ecs_data_usage::*;
use fruits_ecs_system::{SystemInput, SystemParam};

pub struct ExclusiveWorldAccess<'d> {
    world: RwLockWriteGuard<'d, WorldData>,
}

impl<'d> ExclusiveWorldAccess<'d> {
    pub fn world(&self) -> &WorldData {
        &*self.world
    }

    pub fn world_mut(&mut self) -> &mut WorldData {
        &mut *self.world
    }
}

impl<'d> Deref for ExclusiveWorldAccess<'d> {
    type Target = WorldData;

    fn deref(&self) -> &Self::Target {
        &*self.world
    }
}

impl<'d> DerefMut for ExclusiveWorldAccess<'d> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.world
    }
}

unsafe impl<'b> SystemParam for ExclusiveWorldAccess<'b> {
    type Item<'d> = ExclusiveWorldAccess<'d>;

    fn fill_data_usage(usage: &mut DataUsage) {
        usage.add_all_mut();
    }

    fn new<'d>(input: SystemInput<'d>) -> Option<Self::Item<'d>> {
        let guard = input.world_data.try_write().ok()?;

        Some(ExclusiveWorldAccess {
            world: guard,
        })
    }
}