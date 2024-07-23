use std::{ops::{Deref, DerefMut}, sync::{RwLock, RwLockWriteGuard}};

use crate::{data::world_data::WorldData, data_usage::WorldSharedDataUsage, system::SystemParam, system_data::SystemStatesHolder};

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

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        usage.add_all_mut();
    }

    fn new<'d>(data: &'d RwLock<WorldData>, system_data: &'d SystemStatesHolder) -> Option<Self::Item<'d>> {
        let guard = data.try_write().ok()?;

        Some(ExclusiveWorldAccess {
            world: guard,
        })
    }
}