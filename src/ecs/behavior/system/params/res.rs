use std::{
    any::TypeId,
    ops::Deref,
    sync::{RwLock, RwLockReadGuard},
};

use crate::{
    ecs::{
        behavior::{
            system::SystemParam, world_behavior::WorldSharedDataUsage, SingleWorldSharedDataUsage,
        },
        data::{
            resources::Resource,
            world_data::WorldData
        },
    },
    mapped_locks::MappedRwLockReadGuard,
};

pub struct Res<'w, R: 'w + Resource> {
    resource: MappedRwLockReadGuard<'w, WorldData, RwLockReadGuard<'w, R>>,
}

impl<'w, R: Resource> Deref for Res<'w, R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}

unsafe impl<'a, R: Resource> SystemParam for Res<'a, R> {
    type Item<'b> = Res<'b, R>;

    fn fill_data_usage(usage: &mut WorldSharedDataUsage) {
        usage.add(SingleWorldSharedDataUsage::new_readonly(TypeId::of::<
            R,
        >()));
    }

    fn from_world_data<'w>(data: &'w RwLock<WorldData>) -> Option<Self::Item<'w>> {
        let guard = data.try_read().ok()?;

        Some(Res {
            resource: MappedRwLockReadGuard::try_map_from(guard, |w| {
                w.resources().get::<R>()
            })?,
        })
    }
}
