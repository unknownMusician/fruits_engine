use std::time::Instant;

use fruits_prelude::*;
use fruits_math::{Matrix, Matrix3x3, Vec3};
use fruits_modules::{
    asset::AssetStorageResource,
    render::*,
    transform::GlobalTransform,
};

fn main() {
    let mut app = App::new();

    fruits_modules::render::add_module_to(app.ecs_mut());

    let data = app.ecs_mut().data_mut();

    data.resources_mut().insert(BoidSettings { attraction_threshold: 1.0, damping_factor: 0.2 });


    let systems = app.ecs_mut().behavior_mut();

    systems.get_mut(Schedule::Start).add_system(init);

    let update_systems = systems.get_mut(Schedule::Update);

    update_systems.add_system(accumulate_boid_separation);
    update_systems.add_system(affect_motor_by_boid);
    update_systems.add_system(apply_motor);
    update_systems.add_system(apply_damping);
    update_systems.add_system(apply_velocity);
    update_systems.add_system(restrict_boids);
    update_systems.add_system(rotate_boids_by_velocity);

    update_systems.order_systems(accumulate_boid_separation, affect_motor_by_boid);
    update_systems.order_systems(affect_motor_by_boid, apply_motor);
    update_systems.order_systems(apply_motor, apply_damping);
    update_systems.order_systems(apply_damping, apply_velocity);
    update_systems.order_systems(apply_velocity, rotate_boids_by_velocity);
    update_systems.order_systems(rotate_boids_by_velocity, restrict_boids);

    app.run();
}

#[derive(Component)]
struct Boid {
    target_direction: Vec3<f32>,
}

#[derive(Component)]
struct Velocity(pub Vec3<f32>);

#[derive(Component)]
struct Motor {
    pub acceleration_direction: Vec3<f32>,
    pub strength: f32,
}

#[derive(Component)]
struct BoidTarget { }

#[derive(Resource)]
struct BoidSettings {
    pub attraction_threshold: f32,
    pub damping_factor: f32,
}

fn init(mut world: ExclusiveWorldAccess) {
    world.resources_mut().insert(AssetStorageResource::<Material>::new());
    world.resources_mut().insert(AssetStorageResource::<Mesh>::new());

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
            StandardVertex { position: [0.5, 1.0, 0.0], color: [0.2, 0.5, 1.0, 0.0], ..Default::default() },
            StandardVertex { position: [1.0, 0.0, 0.0], color: [0.0, 0.0, 0.0, 0.0], ..Default::default() },
        ];

        for vertex in vertices.iter_mut() {
            vertex.position[0] -= 0.5_f32;
            vertex.position[1] -= 0.5_f32;

            for dimention in vertex.position.iter_mut() {
                *dimention *= 0.2_f32;
            }
        }

        let indices = [
            0, 2, 1,
        ];

        let mesh = Mesh::new(device, &vertices, &indices);

        (material, mesh)
    };

    let material = world.resources_mut().get_mut::<AssetStorageResource::<Material>>().unwrap().insert(material);
    let mesh = world.resources_mut().get_mut::<AssetStorageResource::<Mesh>>().unwrap().insert(mesh);
    
    for _ in 0..100 {
        let entity = world.entities_components_mut().create_entity();

        world.entities_components_mut().add_component(entity, RenderMeshComponent { mesh: mesh.clone() });
        world.entities_components_mut().add_component(entity, RenderMaterialComponent { material: material.clone() });
        world.entities_components_mut().add_component(entity, Boid { target_direction: Vec3::with_all(0.0) });
        world.entities_components_mut().add_component(entity, BoidTarget { });
        world.entities_components_mut().add_component(entity, Motor { acceleration_direction: Vec3::with_all(0.0), strength: 0.01 });
        world.entities_components_mut().add_component(entity, Velocity(Vec3::with_all(0.0)));
        world.entities_components_mut().add_component(entity, GlobalTransform {
            scale_rotation: Matrix3x3::IDENTITY,
            position: Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0),
        });

        
    }

    {
        let camera_entity = world.entities_components_mut().create_entity();

        world.entities_components_mut().add_component(camera_entity, GlobalTransform {
            scale_rotation: Matrix::IDENTITY,
            position: Vec3::new(0.0_f32, 0.0_f32, -5.0f32),
        });
        world.entities_components_mut().add_component(camera_entity, CameraComponent {
            near: 0.1_f32,
            far: 1_000_f32,
            fov: 90_f32.to_radians(),
        });
    }
}

fn accumulate_boid_separation(
    boid_settings: Res<BoidSettings>,
    targets_queue: WorldQuery<(Entity, &GlobalTransform, &BoidTarget)>,
    mut boids_queue: WorldQuery<(Entity, &GlobalTransform, &mut Boid)>,
) {
    let timer = Instant::now();

    for (boid_entity, boid_transform, boid) in boids_queue.iter_mut() {
        let mut sum = Vec3::with_all(0.0_f32);

        for (target_entity, target_transform, _) in targets_queue.iter() {
            if target_entity == boid_entity {
                continue;
            }

            let difference = target_transform.position - boid_transform.position;

            let distance = difference.length();

            let attraction_strength = distance as f32 - boid_settings.attraction_threshold;

            sum += difference.normalized_or_0() * attraction_strength;
        }

        boid.target_direction += sum.normalized_or_0();
    }

    println!("{:>5} fps - {:>10.3} ms", (1.0 / timer.elapsed().as_secs_f64()) as u32, timer.elapsed().as_secs_f64() * 1000.0);
}

fn affect_motor_by_boid(
    mut query: WorldQuery<(&Boid, &mut Motor)>
) {
    for (boid, motor) in query.iter_mut() {
        motor.acceleration_direction = (motor.acceleration_direction.normalized_or_0() + boid.target_direction).normalized_or_0();
    }
}

fn apply_motor(
    mut query: WorldQuery<(&Motor, &mut Velocity)>,
) {
    for (motor, velocity) in query.iter_mut() {
        velocity.0 += motor.acceleration_direction.normalized_or_0() * motor.strength;
    }
}

fn apply_damping(
    mut query: WorldQuery<&mut Velocity>,
    boid_settings: Res<BoidSettings>,
) {
    for velocity in query.iter_mut() {
        velocity.0 -= velocity.0 * boid_settings.damping_factor;
    }
}

fn apply_velocity(
    mut query: WorldQuery<(&Velocity, &mut GlobalTransform)>
) {
    for (velocity, transform) in query.iter_mut() {
        transform.position += velocity.0;
    }
}

fn rotate_boids_by_velocity(
    mut query: WorldQuery<(&mut GlobalTransform, &Velocity, &Boid)>,
) {
    for (transform, velocity, _) in query.iter_mut() {
        let angle = f32::atan2(velocity.0.x, velocity.0.y);
        transform.scale_rotation = fruits_math::Matrix3x3::rotation_z(angle)
    }
}

fn restrict_boids(
    mut query: WorldQuery<(&mut GlobalTransform, &Boid)>,
) {
    for (transform, _) in query.iter_mut() {
        transform.position = Vec3::new(
            transform.position.x.clamp(-5.0, 5.0),
            transform.position.y.clamp(-5.0, 5.0),
            transform.position.z.clamp(-5.0, 5.0),
        ); 
    }
}