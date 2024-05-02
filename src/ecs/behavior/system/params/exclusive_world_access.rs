use std::{ops::{Deref, DerefMut}, sync::{RwLock, RwLockWriteGuard}};

use crate::ecs::{behavior::{system::SystemParam, WorldSharedDataUsage}, data::world_data::WorldData};

pub struct ExclusiveWorldAccess<'w> {
    world: RwLockWriteGuard<'w, WorldData>,
}

impl<'w> ExclusiveWorldAccess<'w> {
    pub fn world(&self) -> &WorldData {
        &*self.world
    }

    pub fn world_mut(&mut self) -> &mut WorldData {
        &mut *self.world
    }
}

impl<'w> Deref for ExclusiveWorldAccess<'w> {
    type Target = WorldData;

    fn deref(&self) -> &Self::Target {
        &*self.world
    }
}

impl<'w> DerefMut for ExclusiveWorldAccess<'w> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.world
    }
}

unsafe impl<'b> SystemParam for ExclusiveWorldAccess<'b> {
    type Item<'a> = ExclusiveWorldAccess<'a>;

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        usage.add_all_mut();
    }

    fn from_world_data<'w>(data: &'w RwLock<WorldData>) -> Option<Self::Item<'w>> {
        let guard = data.try_write().ok()?;

        Some(ExclusiveWorldAccess {
            world: guard,
        })
    }
}