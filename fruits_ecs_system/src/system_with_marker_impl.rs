use fruits_ecs_data_usage::DataUsage;

use crate::{
    system::System,
    system_input::SystemInput,
    system_param::SystemParam,
    system_with_marker::SystemWithMarker,
    system_with_marker_adapter::SystemWithMarkerAdapter,
};

macro_rules! system_with_marker_impl {
    ($($P: ident),*) => {
        unsafe impl<F, $($P),*> SystemWithMarker<fn($($P),*)> for F
        where
            for<'a> F: 'static + Send + Sync + Fn($($P),*) + Fn($($P::Item<'a>),*),
            fn($($P),*): 'static,
            $($P: SystemParam),*
        {
            #[allow(redundant_semicolons)]
            fn fill_data_usage(&self, _usage: &mut DataUsage) {
                $($P::fill_data_usage(_usage));*;
            }
        
            fn execute<'d>(&self, _data: SystemInput<'d>) {
                self(
                    $($P::new(_data).unwrap_or_else(|| panic!(
                        "System cannot obtain its parameters. System: {}. Parameter: {}.",
                        std::any::type_name::<F>(),
                        std::any::type_name::<$P>(),
                    ))),*
                );
            }

            fn into_system_generic(self) -> Box<dyn System> {
                Box::new(SystemWithMarkerAdapter::new(Box::new(self)))
            }

            fn system_name(&self) -> &'static str {
                std::any::type_name::<F>()
            }
        }
    };
}

system_with_marker_impl!();
system_with_marker_impl!(P0);
system_with_marker_impl!(P0, P1);
system_with_marker_impl!(P0, P1, P2);
system_with_marker_impl!(P0, P1, P2, P3);
system_with_marker_impl!(P0, P1, P2, P3, P4);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6, P7);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
system_with_marker_impl!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);