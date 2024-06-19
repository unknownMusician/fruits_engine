use std::marker::PhantomData;

use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, Color, CommandEncoderDescriptor, IndexFormat, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, ShaderStages, StoreOp, SurfaceTexture, TextureViewDescriptor};

use crate::{app::RenderStateResource, ecs::{behavior::Schedule, data::{archetypes::component::Component, resources::Resource}, WorldBuilder}, math::{self, Matrix4x4}, models::{shader::Shader, Material, Mesh}, tools::index_version_collection::{VersionCollection, VersionIndex}, ExclusiveWorldAccess, Res, ResMut, WorldQuery};

use super::transforms_module::GlobalTransform;

pub fn add_module_to(world: &mut WorldBuilder) {
    world.data_mut().resources_mut().insert(SurfaceTextureResource { texture: None, });
    
    world.behavior_mut().get_mut(Schedule::Start).add_system(create_camera_uniform_buffer);
    world.behavior_mut().get_mut(Schedule::Update).add_system(update_camera_uniform_buffer);
    world.behavior_mut().get_mut(Schedule::Update).add_system(request_surface_texture_view);
    world.behavior_mut().get_mut(Schedule::Update).add_system(render_meshes_and_materials);
    world.behavior_mut().get_mut(Schedule::Update).add_system(present_surface);

    world.behavior_mut().get_mut(Schedule::Update).order_systems(update_camera_uniform_buffer, render_meshes_and_materials);
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

pub struct CameraComponent;
impl Component for CameraComponent { }

pub struct SurfaceTextureResource {
    texture: Option<SurfaceTexture>,
}
impl Resource for SurfaceTextureResource { }

pub struct UniformBindGroupResource {
    layout: Option<BindGroupLayout>,
    bind_group: Option<BindGroup>,
}
impl Resource for UniformBindGroupResource { }

pub struct CameraUniformBufferResource {
    buffer: Buffer,
    layout: BindGroupLayout,
    group: BindGroup,
}
impl Resource for CameraUniformBufferResource { }

pub fn create_uniform_bind_group(

) {
    
}

pub fn create_camera_uniform_buffer(
    mut world: ExclusiveWorldAccess,
    render_state: Res<RenderStateResource>,
    mut buffer: ResMut<CameraUniformBufferResource>,
) {
    let render_state = world.resources().get::<RenderStateResource>().unwrap().get().lock().unwrap();

    let buffer = render_state.device().create_buffer_init(&BufferInitDescriptor {
        label: Some("Camera Buffer"),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        contents: unsafe { (&Matrix4x4::<f32>::identity().into_array()).align_to::<u8>().1 },
    });

    let bind_group_layout = render_state.device().create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Camera bind group layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ]
    });

    let bind_group = render_state.device().create_bind_group(&BindGroupDescriptor {
        label: Some("Camera bind group"),
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            },
        ],
    });

    world.resources_mut().insert(CameraUniformBufferResource {
        buffer,
        layout: bind_group_layout,
        group: bind_group,
    });
}

pub fn update_camera_uniform_buffer(
    render_state: Res<RenderStateResource>,
    buffer: ResMut<CameraUniformBufferResource>,
    query: WorldQuery<(&GlobalTransform, &CameraComponent)>
) {
    if query.len() != 1 {
        panic!("There should be one and only camera in the world.");
    }

    let (transform, _) = query.iter().next().unwrap();

    let matrix = math::into_matrix4x4_with_pos(transform.scale_rotation, transform.position);

    let matrix = matrix.into_array();
    let matrix = unsafe { matrix.align_to::<u8>().1 };

    render_state.get().lock().unwrap().queue().write_buffer(&buffer.buffer, 0, matrix);
}

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
    camera_buffer: Res<CameraUniformBufferResource>,
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
            render_pass.set_bind_group(1, &camera_buffer.group, &[]);
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