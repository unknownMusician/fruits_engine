use std::sync::{Arc, Mutex};

use crate::render_app_state::RenderAppState;

pub struct RenderAppStateHolder {
    state: Mutex<Option<Arc<RenderAppState>>>,
}

impl RenderAppStateHolder {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(None),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.state.lock().unwrap().is_some()
    }

    pub fn init(&self, state: Arc<RenderAppState>) {
        let mut old_state = self.state.lock().unwrap();

        if old_state.is_some() {
            panic!("RenderAppStateHolder is already initialized.");
        }

        *old_state = Some(state);
    }

    pub fn get(&self) -> Arc<RenderAppState> {
        Arc::clone(self.state.lock().unwrap().as_ref().unwrap())
    }
}