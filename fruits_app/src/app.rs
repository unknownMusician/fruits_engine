use std::sync::{Arc, Mutex};

use fruits_ecs_schedule::Schedule;
use fruits_ecs_world::WorldBuilder;

use crate::{render_app::run_render_app, render_state_resource::RenderStateResource};

pub struct App {
    ecs_world: WorldBuilder,
}

impl App {
    pub fn new() -> Self {
        Self {
            ecs_world: WorldBuilder::new(),
        }
    }

    pub fn ecs(&self) -> &WorldBuilder {
        &self.ecs_world
    }

    pub fn ecs_mut(&mut self) -> &mut WorldBuilder {
        &mut self.ecs_world
    }

    pub fn run(mut self) {
        let world_initializer = Arc::new(Mutex::new(None));
        let world_access = Arc::clone(&world_initializer);

        run_render_app(
            move |render_state| {
                let render_state = RenderStateResource::new(Arc::clone(render_state));

                self.ecs_mut().data_mut().resources_mut().insert(render_state);
        
                let world = self.ecs_world.build();
        
                world.execute_iteration(Schedule::Start);

                *world_initializer.lock().unwrap() = Some(world);
            },
            move || world_access.lock().unwrap().as_ref().unwrap().execute_iteration(Schedule::Update),
        );
    }
}