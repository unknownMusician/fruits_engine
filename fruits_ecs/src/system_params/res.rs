use std::{
    any::TypeId, ops::Deref, sync::{RwLock, RwLockReadGuard}
};

use crate::{
    data::{
        resource::Resource,
        world_data::WorldData
    }, data_usage::{SingleWorldSharedDataUsage, WorldSharedDataUsage}, system::SystemParam, system_data::SystemStatesHolder
};

use fruits_utils::mapped_guard::{MappedGuard, RwLockReadGuarding};

pub struct Res<'w, R: Resource> {
    resource: MappedGuard<'w, RwLockReadGuarding, WorldData, RwLockReadGuard<'w, R>>,
}

impl<'w, R: Resource> Deref for Res<'w, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

struct Test<'d, R: Resource> {
    guard: RwLockReadGuard<'d, WorldData>,
    r: Option<RwLockReadGuard<'d, R>>,
}

unsafe impl<'a, R: Resource> SystemParam for Res<'a, R> {
    type Item<'d> = Res<'d, R>;

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        usage.add(SingleWorldSharedDataUsage::new_readonly(TypeId::of::<R>()));
    }

    fn new<'d>(data: &'d RwLock<WorldData>, system_data: &'d SystemStatesHolder) -> Option<Self::Item<'d>> {
        let guard = data.try_read().ok()?;

        // todo: remove MappedRwLockGuard
        Some(Res {
            resource: MappedGuard::<'_, _, WorldData, _>::try_map_from(guard, |w| {
                w.resources().get::<R>()
            })?,
        })
    }
}
