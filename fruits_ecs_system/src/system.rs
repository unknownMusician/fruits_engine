use fruits_ecs_data_usage::DataUsage;

use crate::system_input::SystemInput;

pub unsafe trait System : 'static + Send + Sync {
    fn fill_data_usage(&self, usage: &mut DataUsage);
    fn execute<'d>(&self, data: SystemInput<'d>);
    fn system_name(&self) -> &'static str;
}