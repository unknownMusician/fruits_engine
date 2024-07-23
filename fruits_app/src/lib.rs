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

    pub fn run(mut self, event_loop_final_handler: impl FnOnce() + 'static) {
        let render_app = RenderApp::new();

        let render_state = RenderStateResource::new(Arc::clone(render_app.state()));

        self.ecs_world_mut().data_mut().resources_mut().insert(render_state);

        let world = self.ecs_world.build();

        world.execute_iteration(Schedule::Start);

        render_app.run(
            move || world.execute_iteration(Schedule::Update),
            event_loop_final_handler
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