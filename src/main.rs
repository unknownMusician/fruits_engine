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

use std::{fs, thread};

use app::App;
use ecs::{
    behavior::system::params::*,
    data::{
        archetypes::component::Component,
        resources::Resource,
    },
};

use crate::{app::RenderStateResource, ecs::behavior::Schedule, ecs_modules::render_module::{self, AssetStorageResource, RenderMaterialComponent, RenderMeshComponent}, models::{Material, Mesh}};

// fn run_ecs_data_integration_test() {
//     let mut world = ecs::World::new();

//     let entity = world.data_mut().create_entity();
//     println!("[info] created entity {}.", entity.index);

//     let add_component_result = world.data_mut().add_component(entity, 5_i32);
//     println!("[info] created entity {}: {}", entity.index, add_component_result.is_none());

//     println!("[check] is entity {} contains component i32: {}", entity.index, world.data_mut().get_component_ref::<i32>(entity).is_some());
//     println!("[check] does entity {} exist: {}", entity.index, world.data_mut().entity_exists(entity));

//     world.data_mut().destroy_entity(entity);
//     println!("[info] deleted entity {}.", entity.index);

//     println!("[check] does entity {} exist: {}", entity.index, world.data_mut().entity_exists(entity));

//     let entity = world.data_mut().create_entity();
//     println!("[info] created entity {}.", entity.index);

//     println!("[check] does entity {} exist: {}", entity.index, world.data_mut().entity_exists(entity));
// }

fn run_ecs_behavior_integration_test() {
    let mut app = App::new();
    let world = app.ecs_world_mut();

    render_module::add_module_to(world);

    //world.behavior_mut().get_mut(Schedule::Update).add_system(resource_system1);
    //world.behavior_mut().get_mut(Schedule::Update).add_system(resource_system2);
    world.behavior_mut().get_mut(Schedule::Start).add_system(init_resources);
    world.behavior_mut().get_mut(Schedule::Start).add_system(init_mesh_material);
    world.behavior_mut().get_mut(Schedule::Update).add_system(create_entity);
    world.behavior_mut().get_mut(Schedule::Start).order_systems(init_resources, init_mesh_material);
    //world.behavior_mut().get_mut(Schedule::Update).order_systems(sample_system2, sample_system1);

    world.data_mut().entities_components_mut().create_entity();

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

fn create_entity(mut world: ExclusiveWorldAccess) {
    for _ in 0..10_000 {
        world.entities_components_mut().create_entity();
    }
    let count = world.entities_components_mut().entities_count();

    println!("Entities count: {}; Bytes allocated: {}", count, alloc_monitor::allocated());
}

fn main() {
    run_ecs_behavior_integration_test();
}
