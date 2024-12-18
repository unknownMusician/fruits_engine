use fruits_ecs_data_usage::DataUsage;

use crate::system_input::SystemInput;

pub unsafe trait SystemParam {
    type Item<'d> : 'd + SystemParam;

    fn fill_data_usage(usage: &mut DataUsage);
    fn new<'d>(input: SystemInput<'d>) -> Option<Self::Item<'d>>;
}