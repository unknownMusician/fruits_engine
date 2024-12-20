use std::time::Instant;

use fruits_prelude::*;
use fruits_math::{Matrix, Matrix3x3, Vec3};
use fruits_modules::{asset::AssetStorageResource, render::{self as render_module, assets::*, components::*, resources::*, systems::create_camera_uniform_bind_group_layout}, transform::GlobalTransform};

fn main() {
    run_ecs_behavior_integration_test();
}

fn run_ecs_behavior_integration_test() {
    let mut app = App::new();
    let world = app.ecs_mut();

    render_module::add_module_to(world);

    world.behavior_mut().get_mut(Schedule::Start).add_system(init_resources);
    world.behavior_mut().get_mut(Schedule::Start).add_system(init_mesh_material);
    world.behavior_mut().get_mut(Schedule::Update).add_system(update_time);
    world.behavior_mut().get_mut(Schedule::Update).add_system(move_camera);
    world.behavior_mut().get_mut(Schedule::Update).add_system(move_cube);
    world.behavior_mut().get_mut(Schedule::Update).add_system(log_fps);

    world.behavior_mut().get_mut(Schedule::Update).order_systems(move_cube, move_camera);
    world.behavior_mut().get_mut(Schedule::Start).order_systems(init_resources, init_mesh_material);
    world.behavior_mut().get_mut(Schedule::Start).order_systems(create_camera_uniform_bind_group_layout, init_mesh_material);

    let entity = world.data_mut().entities_components_mut().create_entity();

    world.data_mut().entities_components_mut().add_component(entity, GlobalTransform {
        scale_rotation: Matrix::IDENTITY,
        position: Vec3::new(0.0_f32, 0.0_f32, -1.0f32),
    });
    world.data_mut().entities_components_mut().add_component(entity, CameraComponent {
        near: 0.1_f32,
        far: 1_000_f32,
        fov: 90_f32.to_radians(),
    });

    dbg!(world.data_mut().entities_components_mut().entities_count());

    println!("start");
    app.run();
    println!("end");
}

#[derive(Resource)]
struct SampleResource {}

#[derive(Component)]
struct MovingCubeComponent;

#[derive(Resource)]
struct TimeResource {
    pub time: f32,
    pub start: Option<Instant>,
}

#[derive(Resource)]
struct FpsResource {
    pub last_measure_seconds: usize,
    pub count: usize,
}

fn init_resources(mut world: ExclusiveWorldAccess) {
    world.resources_mut().insert(SampleResource { });
    world.resources_mut().insert(FpsResource { last_measure_seconds: 0, count: 0 });
    world.resources_mut().insert(TimeResource { time: 0.0_f32, start: None });

    world.resources_mut().insert(AssetStorageResource::<Mesh>::new());
    world.resources_mut().insert(AssetStorageResource::<Material>::new());
}

fn init_mesh_material(mut world: ExclusiveWorldAccess) {
    let (material, mesh) = {
        let camera_group_layout = &*world.resources().get::<CameraUniformBufferGroupLayoutResource>().unwrap();
        let render_state = world.resources().get::<RenderStateResource>().unwrap();

        let device = render_state.device();
        let surface_config = &*render_state.surface_config().lock().unwrap();

        let shader_code = include_str!("./../../../src/shader.wgsl");
        let shader = Shader::new_wgsl(device, shader_code);

        let bind_group_layouts = [
            camera_group_layout.layout(),
        ];

        let material = Material::new(device, surface_config, &shader, &bind_group_layouts);

        let mut vertices = [
            StandardVertex { position: [0.0, 0.0, 0.0], color: [0.0, 0.0, 0.0, 0.0], ..Default::default() },
            StandardVertex { position: [1.0, 0.0, 0.0], color: [1.0, 0.0, 0.0, 0.0], ..Default::default() },
            StandardVertex { position: [0.0, 1.0, 0.0], color: [0.0, 1.0, 0.0, 0.0], ..Default::default() },
            StandardVertex { position: [1.0, 1.0, 0.0], color: [1.0, 1.0, 0.0, 0.0], ..Default::default() },
            StandardVertex { position: [0.0, 0.0, 1.0], color: [0.0, 0.0, 1.0, 0.0], ..Default::default() },
            StandardVertex { position: [1.0, 0.0, 1.0], color: [1.0, 0.0, 1.0, 0.0], ..Default::default() },
            StandardVertex { position: [0.0, 1.0, 1.0], color: [0.0, 1.0, 1.0, 0.0], ..Default::default() },
            StandardVertex { position: [1.0, 1.0, 1.0], color: [1.0, 1.0, 1.0, 0.0], ..Default::default() },
        ];

        for vertex in vertices.iter_mut() {
            for ele in vertex.position.iter_mut() {
                *ele = *ele * 2.0 - 1.0;
                *ele *= 0.2_f32;
            }

            //vertex.position[2] += 3.0_f32;
            //vertex.position[0] += 1.0_f32;
        }

        let indices = [
            0, 1, 3,
            0, 3, 2,
            0, 4, 5,
            0, 5, 1,
            0, 6, 4,
            0, 2, 6,
            1, 7, 3,
            1, 5, 7,
            2, 7, 6,
            2, 3, 7,
            4, 6, 7,
            4, 7, 5,
        ];

        let mesh = Mesh::new(device, &vertices, &indices);

        (material, mesh)
    };

    let material = world.resources_mut().get_mut::<AssetStorageResource::<Material>>().unwrap().insert(material);
    let mesh = world.resources_mut().get_mut::<AssetStorageResource::<Mesh>>().unwrap().insert(mesh);
    
    for _ in 0..3 {
        let entity = world.entities_components_mut().create_entity();
        println!("created entity: i={}, v={}", entity.version_index().index, entity.version_index().version);
        world.entities_components_mut().add_component(entity, RenderMeshComponent { mesh: mesh.clone() });
        world.entities_components_mut().add_component(entity, RenderMaterialComponent { material: material.clone() });
        world.entities_components_mut().add_component(entity, GlobalTransform { scale_rotation: Matrix3x3::IDENTITY, position: Vec3::with_all(0.0) });
        world.entities_components_mut().add_component(entity, MovingCubeComponent);
    }
}

fn update_time(
    mut time: ResMut<TimeResource>,
) {
    let start = match time.start {
        Some(start) => start,
        None => {
            let new_start = Instant::now();
            time.start = Some(new_start);
            new_start
        },
    };

    time.time = start.elapsed().as_secs_f32();
}

fn move_camera(
    time: Res<TimeResource>,
    query: WorldQuery<(&mut GlobalTransform, &CameraComponent)>,
) {
    for (transform, _) in query.iter() {
        //transform.position.x = time.time.sin() * 0.5_f32;
        //transform.position.y = time.time.cos() * 0.5_f32;
        //transform.position.z = -3.0_f32 + time.time.cos() * 10.0_f32;

        //transform.scale_rotation = Matrix3x3::rotation_y(time.time);
    }
}

fn move_cube(
    time: Res<TimeResource>,
    query: WorldQuery<(Entity, &mut GlobalTransform, &MovingCubeComponent)>,
) {
    for (entity, transform, _) in query.iter() {
        let i = entity.version_index().index;

        if i == 1 {
            transform.position.x = time.time.cos() * 0.5_f32;
        } else if i == 2 {
            transform.position.y = time.time.cos() * 0.5_f32;
        } else if i == 3 {
            transform.position.z = time.time.cos() * 0.5_f32;
        }
        transform.scale_rotation = Matrix3x3::rotation_y(time.time * 1.0_f32 * i as f32) * Matrix3x3::rotation_z(-45.0_f32.to_radians());

        //transform.position.x = time.time.sin() * 0.5_f32;
        //transform.position.y = time.time.cos() * 0.5_f32;
    }
}

fn log_fps(
    mut fps: ResMut<FpsResource>,
    time: Res<TimeResource>,
) {
    fps.count += 1;

    if time.time - fps.last_measure_seconds as f32 >= 1.0_f32 {
        println!("fps: {}", fps.count);
        fps.last_measure_seconds = time.time.floor() as usize;

        fps.count = 0;
    }
}