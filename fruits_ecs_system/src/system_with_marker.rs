use fruits_ecs_data_usage::DataUsage;

use crate::{system::System, system_input::SystemInput};

pub unsafe trait SystemWithMarker<M: 'static> : 'static + Send + Sync {
    fn fill_data_usage(&self, usage: &mut DataUsage);
    fn execute<'d>(&self, data: SystemInput<'d>);
    fn into_system_generic(self) -> Box<dyn System>;
    fn system_name(&self) -> &'static str;
}