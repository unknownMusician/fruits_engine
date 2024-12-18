use std::{ops::Deref, sync::Arc};

use fruits_ecs_resource::Resource;
use fruits_ecs_macros::Resource;

use crate::render_app_state::RenderAppState;

#[derive(Resource)]
pub struct RenderStateResource(Arc<RenderAppState>);

impl RenderStateResource {
    pub fn new(state: Arc<RenderAppState>) -> Self {
        Self(state)
    }
}

impl Deref for RenderStateResource {
    type Target = Arc<RenderAppState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}