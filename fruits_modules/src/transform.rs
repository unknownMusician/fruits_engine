use fruits_ecs::data::archetypes::component::Component;
use fruits_math::{
    Matrix3x3,
    Vec3,
};

pub struct GlobalTransform {
    pub scale_rotation: Matrix3x3<f32>,
    pub position: Vec3<f32>,
}
impl Component for GlobalTransform { }