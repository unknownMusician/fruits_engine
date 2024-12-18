use fruits_ecs_component::WorldEntitiesComponents;
use fruits_ecs_resource::ResourcesHolder;

pub struct WorldData {
    resources: ResourcesHolder,
    entities_components: WorldEntitiesComponents,
}

impl WorldData {
    pub fn new() -> Self {
        Self {
            resources: ResourcesHolder::new(),
            entities_components: WorldEntitiesComponents::new(),
        }
    }

    pub fn resources(&self) -> &ResourcesHolder {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut ResourcesHolder {
        &mut self.resources
    }

    pub fn entities_components(&self) -> &WorldEntitiesComponents {
        &self.entities_components
    }

    pub fn entities_components_mut(&mut self) -> &mut WorldEntitiesComponents {
        &mut self.entities_components
    }
}