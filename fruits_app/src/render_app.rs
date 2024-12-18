use std::sync::Arc;

use winit::event_loop::EventLoop;

use crate::{event_loop_handler::EventLoopHandler, render_app_state::RenderAppState};

pub fn run_render_app(
    event_loop_start_handler: impl FnOnce(&Arc<RenderAppState>) + 'static,
    event_loop_update_handler: impl FnMut() + 'static,
) {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut event_loop_handler = EventLoopHandler::new(
        event_loop_start_handler,
        event_loop_update_handler,
    );

    event_loop.run_app(&mut event_loop_handler).unwrap();
}