use std::marker::PhantomData;

use fruits_ecs::data::resource::Resource;
use fruits_utils::index_version_collection::{
    VersionCollection,
    VersionIndex,
};



pub struct AssetWithVersion<T> {
    pub asset: T,
    pub version: usize,
}

pub struct AssetStorageResource<T> {
    assets: VersionCollection<T>,
}
impl<T: 'static + Send + Sync> Resource for AssetStorageResource<T> { }

#[derive(Copy, Clone)]
pub struct AssetHandle<T> {
    index: VersionIndex,
    _phantom: PhantomData<fn(T) -> T>,
}

impl<T> AssetHandle<T> {
    pub fn new(index: VersionIndex) -> Self {
        Self {
            index,
            _phantom: Default::default(),
        }
    }
}

impl<T> AssetStorageResource<T> {
    pub fn new() -> Self {
        Self {
            assets: VersionCollection::new(),
        }
    }

    pub fn insert(&mut self, asset: T) -> AssetHandle<T> {
        let index = self.assets.insert(asset);

        AssetHandle::<T>::new(index)
    }

    pub fn get(&self, handle: &AssetHandle<T>) -> Option<&T> {
        self.assets.get(handle.index)
    }

    pub fn remove(&mut self, handle: &AssetHandle<T>) -> Option<T> {
        self.assets.remove(handle.index)
    }
}
