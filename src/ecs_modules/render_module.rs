use std::marker::PhantomData;

use wgpu::{Color, CommandEncoderDescriptor, IndexFormat, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, StoreOp, SurfaceTexture, TextureView, TextureViewDescriptor};

use crate::{app::RenderStateResource, ecs::{behavior::Schedule, data::{archetypes::component::Component, resources::Resource}, WorldBuilder}, models::{shader::Shader, Material, Mesh}, tools::index_version_collection::{VersionCollection, VersionIndex}, Res, ResMut, WorldQuery};

pub fn add_module_to(world: &mut WorldBuilder) {
    world.data_mut().resources_mut().insert(SurfaceTextureResource { texture: None, });
    
    world.behavior_mut().get_mut(Schedule::Update).add_system(request_surface_texture_view);
    world.behavior_mut().get_mut(Schedule::Update).add_system(render_meshes_and_materials);
    world.behavior_mut().get_mut(Schedule::Update).add_system(present_surface);
    world.behavior_mut().get_mut(Schedule::Update).order_systems(request_surface_texture_view, present_surface);
    world.behavior_mut().get_mut(Schedule::Update).order_systems(request_surface_texture_view, render_meshes_and_materials);
    world.behavior_mut().get_mut(Schedule::Update).order_systems(render_meshes_and_materials, present_surface);
}

pub struct RenderMeshComponent {
    pub mesh: AssetHandle<Mesh>,
}
impl Component for RenderMeshComponent { }

pub struct RenderMaterialComponent {
    pub material: AssetHandle<Material>,
}
impl Component for RenderMaterialComponent { }

pub struct SurfaceTextureResource {
    texture: Option<SurfaceTexture>,
}
impl Resource for SurfaceTextureResource { }

pub fn request_surface_texture_view(
    render_state: Res<RenderStateResource>,
    mut surface_texture: ResMut<SurfaceTextureResource>,
) {
    let render_state = render_state.get().lock().unwrap();
    
    surface_texture.texture = render_state.surface().get_current_texture().ok();
}

pub fn present_surface(mut surface_texture: ResMut<SurfaceTextureResource>) {
    if let Some(texture) = surface_texture.texture.take() {
        texture.present();
    }
}

pub fn render_meshes_and_materials(
    q: WorldQuery<(&RenderMeshComponent, &RenderMaterialComponent)>,
    render_state: Res<RenderStateResource>,
    surface_texture: ResMut<SurfaceTextureResource>,
    meshes: Res<AssetStorageResource<Mesh>>,
    materials: Res<AssetStorageResource<Material>>,
) {
    if q.len() == 0 {
        return;
    }

    let Some(surface_texture) = &surface_texture.texture else { return; }; 

    let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

    let render_state = render_state.get().lock().unwrap();

    for (render_mesh, render_material) in q.iter() {
        let Some(mesh) = meshes.get(&render_mesh.mesh) else { continue; };
        let Some(material) = materials.get(&render_material.material) else { continue; };

        let mut encoder = render_state.device().create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
    
            render_pass.set_pipeline(material.render_pipeline());
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
            render_pass.set_index_buffer(mesh.index_buffer().slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..(mesh.indices_count() as u32), 0, 0..1);
        }
        
        render_state.queue().submit(std::iter::once(encoder.finish()));
    }

}

pub struct AssetWithVersion<T> {
    pub asset: T,
    pub version: usize,
}

pub struct AssetStorageResource<T> {
    assets: VersionCollection<T>,
}
impl<T: 'static + Send + Sync> Resource for AssetStorageResource<T> { }

#[derive(Copy, Clone)]
pub struct AssetHandle<T> {
    index: VersionIndex,
    _phantom: PhantomData<fn(T) -> T>,
}

impl<T> AssetHandle<T> {
    pub fn new(index: VersionIndex) -> Self {
        Self {
            index,
            _phantom: Default::default(),
        }
    }
}

impl<T> AssetStorageResource<T> {
    pub fn new() -> Self {
        Self {
            assets: VersionCollection::new(),
        }
    }

    pub fn insert(&mut self, asset: T) -> AssetHandle<T> {
        let index = self.assets.insert(asset);

        AssetHandle::<T>::new(index)
    }

    pub fn get(&self, handle: &AssetHandle<T>) -> Option<&T> {
        self.assets.get(handle.index)
    }

    pub fn remove(&mut self, handle: &AssetHandle<T>) -> Option<T> {
        self.assets.remove(handle.index)
    }
}

pub fn create_shader(render_state: &RenderStateResource, shader_code: String) -> Shader {
    let render_state = render_state.get().lock().unwrap();

    Shader::new_wgsl(render_state.device(), shader_code)
}

pub fn create_material(render_state: &RenderStateResource, shader: &Shader) -> Material {
    let render_state = render_state.get().lock().unwrap();

    Material::new(render_state.device(), render_state.surface_config(), shader)
}

pub fn create_mesh(render_state: &RenderStateResource) -> Mesh {
    let render_state = render_state.get().lock().unwrap();

    Mesh::new(render_state.device())
}