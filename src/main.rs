mod tools;
mod models;
mod ecs;
mod mapped_locks;
mod quadtree;
mod same;
mod thread_pool;
mod timer;
mod app;
mod ecs_modules;
mod rendering;
mod math;
mod alloc_monitor;

// use std::cell::RefCell;

use app::App;
use ecs::{
    behavior::system::params::*,
    data::{
        archetypes::component::Component,
        resources::Resource,
    },
};
use ecs_modules::transforms_module::GlobalTransform;
use math::{Matrix3x3, Vec3};

use crate::{app::RenderStateResource, ecs::behavior::Schedule, ecs_modules::render_module::{self, AssetStorageResource, CameraComponent, RenderMaterialComponent, RenderMeshComponent}, models::{Material, Mesh}};

fn run_ecs_behavior_integration_test() {
    let mut app = App::new();
    let world = app.ecs_world_mut();

    render_module::add_module_to(world);

    world.behavior_mut().get_mut(Schedule::Start).add_system(init_resources);
    world.behavior_mut().get_mut(Schedule::Start).add_system(init_mesh_material);

    world.behavior_mut().get_mut(Schedule::Start).order_systems(init_resources, init_mesh_material);

    let entity = world.data_mut().entities_components_mut().create_entity();

    world.data_mut().entities_components_mut().add_component(entity, GlobalTransform { scale_rotation: Matrix3x3::identity(), position: Vec3::with_all(0.0_f32) });
    world.data_mut().entities_components_mut().add_component(entity, CameraComponent);

    println!("start");
    app.run(|| {
        println!("end");

        // fs::write("D:/timers.json", Timer::export_all_timers_to_json()).unwrap();
    });
}

struct TimeResource {
}

impl Resource for TimeResource { }

struct SampleResource {}

impl Resource for SampleResource {}

struct SampleComponent {}

impl Component for SampleComponent {}

fn init_resources(mut world: ExclusiveWorldAccess) {
    world.resources_mut().insert(SampleResource { });
    world.resources_mut().insert(TimeResource { });

    world.resources_mut().insert(AssetStorageResource::<Mesh>::new());
    world.resources_mut().insert(AssetStorageResource::<Material>::new());
    world.resources_mut().insert(AssetStorageResource::<Material>::new());
}

fn init_mesh_material(mut world: ExclusiveWorldAccess) {
    let (material, mesh) = {
        let render_state = world.resources().get::<RenderStateResource>().unwrap();

        let shader_code = include_str!("./shader.wgsl");
        let shader = render_module::create_shader(&*render_state, String::from(shader_code));
        let material = render_module::create_material(&*render_state, &shader);
        let mesh = render_module::create_mesh(&*render_state);

        (material, mesh)
    };

    let material = world.resources_mut().get_mut::<AssetStorageResource::<Material>>().unwrap().insert(material);
    let mesh = world.resources_mut().get_mut::<AssetStorageResource::<Mesh>>().unwrap().insert(mesh);
    
    let entity = world.entities_components_mut().create_entity();
    world.entities_components_mut().add_component(entity, RenderMeshComponent { mesh });
    world.entities_components_mut().add_component(entity, RenderMaterialComponent { material });
}

fn main() {
    run_ecs_behavior_integration_test();
}
