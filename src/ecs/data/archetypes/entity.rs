use crate::tools::index_version_collection::{
    VersionCollection,
    VersionIndex,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Entity(VersionIndex);

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
