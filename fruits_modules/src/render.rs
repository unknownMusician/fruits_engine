pub mod assets;
pub mod components;
pub mod resources;
pub mod systems;

use fruits_ecs_world::WorldBuilder;
use fruits_ecs_schedule::Schedule;
use resources::SurfaceTextureResource;
use systems::*;

pub fn add_module_to(world: &mut WorldBuilder) {
    world.data_mut().resources_mut().insert(SurfaceTextureResource { texture: None, });
    
    world.behavior_mut().get_mut(Schedule::Start).add_system(create_camera_uniform_buffer);
    world.behavior_mut().get_mut(Schedule::Start).add_system(create_camera_uniform_bind_group_layout);
    world.behavior_mut().get_mut(Schedule::Start).add_system(create_instance_buffer);
    world.behavior_mut().get_mut(Schedule::Update).add_system(update_camera_uniform_buffer);
    world.behavior_mut().get_mut(Schedule::Update).add_system(request_surface_texture);
    world.behavior_mut().get_mut(Schedule::Update).add_system(render_meshes_and_materials);
    world.behavior_mut().get_mut(Schedule::Update).add_system(present_surface);
    
    world.behavior_mut().get_mut(Schedule::Start).order_systems(create_camera_uniform_bind_group_layout, create_camera_uniform_buffer);
    world.behavior_mut().get_mut(Schedule::Update).order_systems(update_camera_uniform_buffer, render_meshes_and_materials);
    world.behavior_mut().get_mut(Schedule::Update).order_systems(request_surface_texture, present_surface);
    world.behavior_mut().get_mut(Schedule::Update).order_systems(request_surface_texture, render_meshes_and_materials);
    world.behavior_mut().get_mut(Schedule::Update).order_systems(render_meshes_and_materials, present_surface);
}
