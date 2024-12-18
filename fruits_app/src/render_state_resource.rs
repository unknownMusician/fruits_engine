use std::sync::Arc;

use fruits_ecs_resource::Resource;
use fruits_ecs_macros::Resource;

use crate::render_app_state::RenderAppState;

#[derive(Resource)]
pub struct RenderStateResource(Arc<RenderAppState>);

impl RenderStateResource {
    pub fn new(state: Arc<RenderAppState>) -> Self {
        Self(state)
    }

    pub fn get(&self) -> &Arc<RenderAppState> {
        &self.0
    }
}