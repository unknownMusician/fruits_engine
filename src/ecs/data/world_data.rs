use super::{archetypes::world_entities_components::WorldEntitiesComponents, resources::WorldResources};

pub struct WorldData {
    resources: WorldResources,
    entities_components: WorldEntitiesComponents
}

impl WorldData {
    pub fn new() -> Self {
        Self {
            resources: WorldResources::new(),
            entities_components: WorldEntitiesComponents::new(),
        }
    }

    pub fn resources(&self) -> &WorldResources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut WorldResources {
        &mut self.resources
    }

    pub fn entities_components(&self) -> &WorldEntitiesComponents {
        &self.entities_components
    }

    pub fn entities_components_mut(&mut self) -> &mut WorldEntitiesComponents {
        &mut self.entities_components
    }
}