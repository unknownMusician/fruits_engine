use crate::models::{Material, Matrix3x3, Mesh};

pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Self {
        Self { }
    }

    pub fn render(mesh: &Mesh, material: &Material, camera: &Matrix3x3<f32>) {
        
    }
}