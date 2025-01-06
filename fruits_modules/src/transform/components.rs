use fruits_ecs_component::{Component, Entity};
use fruits_ecs_macros::Component;
use fruits_math::{Matrix, Matrix3x3, Quat, Vec3};

#[derive(Component, Copy, Clone)]
pub struct GlobalTransform {
    pub position: Vec3<f32>,
    pub scale_rotation: Matrix3x3<f32>,
}

impl GlobalTransform {
    pub const IDENTITY: GlobalTransform = GlobalTransform {
        position: Vec3::with_all(0.0),
        scale_rotation: Matrix::IDENTITY,
    };
}

#[derive(Component, Copy, Clone)]
pub struct LocalTransform {
    pub position: Vec3<f32>,
    pub rotation: Quat<f32>,
    pub scale: Vec3<f32>,
}

impl LocalTransform {
    pub const IDENTITY: Self = Self {
        position: Vec3::with_all(0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::with_all(1.0),
    };
}

#[derive(Component, Clone)]
pub struct ParentComponent {
    pub children: Vec<Entity>,
}

#[derive(Component, Copy, Clone)]
pub struct ChildComponent {
    pub parent: Entity,
}
