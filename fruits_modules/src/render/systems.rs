use fruits_app::RenderStateResource;
use fruits_ecs_system_params::{ExclusiveWorldAccess, Res, ResMut, WorldQuery};
use fruits_math::{Matrix, Matrix4x4};
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, CommandEncoderDescriptor, IndexFormat, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor, ShaderStages, StoreOp, TextureViewDescriptor};

use crate::{asset::AssetStorageResource, transform::GlobalTransform};

use super::{assets::{Material, Mesh}, components::{CameraComponent, RenderMaterialComponent, RenderMeshComponent}, resources::{CameraUniformBufferGroupLayoutResource, CameraUniformBufferResource, InstanceBufferResource, SurfaceTextureResource}};

pub fn create_camera_uniform_bind_group_layout(
    mut world: ExclusiveWorldAccess,
) {
    let layout = {
        let render_state = world.resources().get::<RenderStateResource>().unwrap();
        let render_state = render_state.get();

        render_state.device().create_bind_group_layout(&BindGroupLayoutDescriptor {
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
        })
    };

    world.resources_mut().insert(CameraUniformBufferGroupLayoutResource::new(layout));
}

pub fn create_camera_uniform_buffer(
    mut world: ExclusiveWorldAccess,
) {
    let (buffer, group) = {
        let layout_resource = &*world.resources().get::<CameraUniformBufferGroupLayoutResource>().unwrap();

        let render_state = world.resources().get::<RenderStateResource>().unwrap();
        let render_state = render_state.get();

        let buffer = render_state.device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: unsafe { (&Matrix4x4::<f32>::IDENTITY.into_array()).align_to::<u8>().1 },
        });

        let group = render_state.device().create_bind_group(&BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: layout_resource.layout(),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
            ],
        });

        (buffer, group)
    };

    world.resources_mut().insert(CameraUniformBufferResource {
        buffer,
        group,
    });
}

pub fn create_instance_buffer(
    mut world: ExclusiveWorldAccess,
) {
    let buffer = {
        let render_state = world.resources().get::<RenderStateResource>().unwrap();
        let render_state = render_state.get();

        let buffer = render_state.device().create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer"),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            contents: unsafe { (&Matrix4x4::<f32>::IDENTITY.into_array()).align_to::<u8>().1 },
        });

        buffer
    };

    world.resources_mut().insert(InstanceBufferResource {
        buffer,
    });
}

pub fn update_camera_uniform_buffer(
    render_state: Res<RenderStateResource>,
    buffer: ResMut<CameraUniformBufferResource>,
    query: WorldQuery<(&GlobalTransform, &CameraComponent)>,
) {
    if query.len() != 1 {
        panic!("There should be one and only camera in the world.");
    }

    let (transform, camera) = query.iter().next().unwrap();

    let projection_matrix = fruits_math::perspective_proj_matrix(camera.fov, camera.near, camera.far);

    let transform_matrix = fruits_math::into_matrix4x4_with_pos(transform.scale_rotation, transform.position).inverse().unwrap();

    let matrix = projection_matrix * transform_matrix;

    let matrix = matrix.into_array();
    let matrix = unsafe { matrix.align_to::<u8>().1 };

    render_state.get().queue().write_buffer(&buffer.buffer, 0, matrix);
}

pub fn request_surface_texture(
    render_state: Res<RenderStateResource>,
    mut surface_texture: ResMut<SurfaceTextureResource>,
) {
    let render_state = render_state.get();
    
    surface_texture.texture = render_state.surface().get_current_texture().ok();
}

pub fn present_surface(mut surface_texture: ResMut<SurfaceTextureResource>) {
    if let Some(texture) = surface_texture.texture.take() {
        texture.present();
    }
}

pub fn render_meshes_and_materials(
    query: WorldQuery<(&GlobalTransform, &RenderMeshComponent, &RenderMaterialComponent)>,
    render_state: Res<RenderStateResource>,
    camera_buffer: Res<CameraUniformBufferResource>,
    instance_buffer: Res<InstanceBufferResource>,
    surface_texture: ResMut<SurfaceTextureResource>,
    meshes: Res<AssetStorageResource<Mesh>>,
    materials: Res<AssetStorageResource<Material>>,
) {
    if query.len() == 0 {
        return;
    }

    let Some(surface_texture) = &surface_texture.texture else { return; }; 

    let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

    let render_state = render_state.get();

    for (transform, render_mesh, render_material) in query.iter() {
        let Some(mesh) = meshes.get(&render_mesh.mesh) else { continue; };
        let Some(material) = materials.get(&render_material.material) else { continue; };

        let transform_matrix = fruits_math::into_matrix4x4_with_pos(transform.scale_rotation, transform.position);
        let transform_matrix = transform_matrix.into_array();
        let transform_matrix = unsafe { transform_matrix.align_to::<u8>().1 };

        render_state.queue().write_buffer(&instance_buffer.buffer, 0, transform_matrix);
        render_state.queue().submit([]);

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
                        load: LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });
    
            render_pass.set_pipeline(material.render_pipeline());
            render_pass.set_bind_group(0, &camera_buffer.group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer().slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..(mesh.indices_count() as u32), 0, 0..1);
        }
        
        render_state.queue().submit(std::iter::once(encoder.finish()));
    }

}