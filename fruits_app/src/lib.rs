use std::sync::{Arc, Mutex};

use fruits_ecs::{data::resource::Resource, world::WorldBuilder, world_behavior::Schedule};
use fruits_window::{RenderApp, RenderAppState};

pub struct App {
    ecs_world: WorldBuilder,
}

impl App {
    pub fn new() -> Self {
        Self {
            ecs_world: WorldBuilder::new(),
        }
    }

    pub fn ecs_world(&self) -> &WorldBuilder {
        &self.ecs_world
    }

    pub fn ecs_world_mut(&mut self) -> &mut WorldBuilder {
        &mut self.ecs_world
    }

    pub fn run(mut self, event_loop_end_handler: impl FnOnce() + 'static) {
        let render_app = RenderApp::new();

        let world_initializer = Arc::new(Mutex::new(None));
        let world_access = Arc::clone(&world_initializer);

        render_app.run(
            move |render_state| {
                let render_state = RenderStateResource::new(Arc::clone(render_state));

                self.ecs_world_mut().data_mut().resources_mut().insert(render_state);
        
                let world = self.ecs_world.build();
        
                world.execute_iteration(Schedule::Start);

                *world_initializer.lock().unwrap() = Some(world);
            },
            move || world_access.lock().unwrap().as_ref().unwrap().execute_iteration(Schedule::Update),
            event_loop_end_handler
        );
    }
}

pub struct RenderStateResource(Arc<Mutex<RenderAppState>>);

impl Resource for RenderStateResource { }

impl RenderStateResource {
    pub fn new(state: Arc<Mutex<RenderAppState>>) -> Self {
        Self(state)
    }

    pub fn get(&self) -> &Arc<Mutex<RenderAppState>> {
        &self.0
    }
}