use fruits_ecs_component::Component;
use fruits_ecs_macros::Component;

use crate::asset::AssetHandle;

use super::assets::{Material, Mesh};

#[derive(Component)]
pub struct RenderMeshComponent {
    pub mesh: AssetHandle<Mesh>,
}

#[derive(Component)]
pub struct RenderMaterialComponent {
    pub material: AssetHandle<Material>,
}

#[derive(Component)]
pub struct CameraComponent {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}
