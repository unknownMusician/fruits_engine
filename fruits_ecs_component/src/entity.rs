use std::fmt::Debug;

use fruits_utils::index_version_collection::{
    VersionCollection,
    VersionIndex,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Entity(VersionIndex);

impl Entity {
    pub const EMPTY: Entity = Entity(VersionIndex { index: 0, version: 0 });

    pub fn version_index(&self) -> VersionIndex {
        self.0
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Entity").field("i", &self.0.index).field("v", &self.0.version).finish()
    }
}

#[derive(Clone, Copy)]
pub struct EntityLocation {
    pub archetype_id: usize,
    pub entity_archetype_index: usize,
}

pub struct WorldEntities(VersionCollection<EntityLocation>);

impl WorldEntities {
    pub fn new() -> Self {
        Self(VersionCollection::new())
    }

    pub fn insert(&mut self, location: EntityLocation) -> Entity {
        Entity(self.0.insert(location))
    }

    pub fn remove(&mut self, entity: Entity) -> Option<EntityLocation> {
        self.0.remove(entity.0)
    }

    pub fn get(&self, entity: Entity) -> Option<&EntityLocation> {
        self.0.get(entity.0)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut EntityLocation> {
        self.0.get_mut(entity.0)
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.0.contains_index(entity.0)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
