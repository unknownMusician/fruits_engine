use fruits_ecs_world::WorldBuilder;
use winit::event_loop::EventLoop;

use crate::event_loop_handler::EventLoopHandler;

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

    pub fn run(self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(&mut EventLoopHandler::new(self.ecs_world)).unwrap();
    }
}