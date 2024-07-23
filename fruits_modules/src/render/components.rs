use fruits_ecs::data::archetypes::component::Component;

use crate::asset::AssetHandle;

use super::assets::{Material, Mesh};

pub struct RenderMeshComponent {
    pub mesh: AssetHandle<Mesh>,
}
impl Component for RenderMeshComponent { }

pub struct RenderMaterialComponent {
    pub material: AssetHandle<Material>,
}
impl Component for RenderMaterialComponent { }

pub struct CameraComponent {
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}
impl Component for CameraComponent { }
