use fruits_ecs_component::Component;
use fruits_ecs_macros::Component;
use fruits_math::{
    Matrix3x3,
    Vec3,
};

#[derive(Component)]
pub struct GlobalTransform {
    pub scale_rotation: Matrix3x3<f32>,
    pub position: Vec3<f32>,
}