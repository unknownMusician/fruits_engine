use std::sync::RwLock;

use crate::ecs::data::world_data::WorldData;

use super::world_behavior::WorldSharedDataUsage;

pub mod params;

pub unsafe trait SystemWithMarker<M> : 'static + Send + Sync {
    fn fill_data_usage(&self, usage: &mut WorldSharedDataUsage);
    fn borrow_and_execute<'w>(&self, data: &'w RwLock<WorldData>);
    fn name(&self) -> &'static str;
    fn into_system_generic(self) -> Box<dyn System>;
}

pub unsafe trait System : 'static + Send + Sync {
    fn fill_data_usage(&self, usage: &mut WorldSharedDataUsage);
    fn borrow_and_execute(&self, data: &RwLock<WorldData>);
    fn name(&self) -> &'static str;
}

struct SystemGeneric<M: 'static> {
    system_with_marker: Box<dyn SystemWithMarker<M>>,
}

pub unsafe trait SystemParam {
    type Item<'a> : 'a + SystemParam;

    fn fill_data_usage(usage: &mut WorldSharedDataUsage);
    fn from_world_data<'w>(data: &'w RwLock<WorldData>) -> Option<Self::Item<'w>>;
}

unsafe impl<M: 'static> System for SystemGeneric<M>
where {
    fn fill_data_usage(&self, usage: &mut WorldSharedDataUsage) {
        self.system_with_marker.fill_data_usage(usage)
    }

    fn borrow_and_execute(&self, data: &RwLock<WorldData>) {
        self.system_with_marker.borrow_and_execute(data)
    }
    
    fn name(&self) -> &'static str {
        self.system_with_marker.name()
    }
}

//

macro_rules! system_with_marker_impl {
    ($($P: ident),*) => {
        unsafe impl<F, $($P),*> SystemWithMarker<fn($($P),*)> for F
        where
            for<'a> F: 'static + Send + Sync + Fn($($P),*) + Fn($($P::Item<'a>),*),
            fn($($P),*): 'static,
            $($P: SystemParam),*
        {
            #[allow(redundant_semicolons)]
            fn fill_data_usage(&self, _usage: &mut WorldSharedDataUsage) {
                $($P::fill_data_usage(_usage));*;
            }
        
            fn borrow_and_execute<'w>(&self, _data: &'w RwLock<WorldData>) {
                self(
                    $($P::from_world_data(_data).expect(&format!("System cannot obrain its parameters. System: {}. Parameter: {}.", std::any::type_name::<F>(), std::any::type_name::<$P>()))),*
                );
            }

            fn name(&self) -> &'static str {
                std::any::type_name::<F>()
            }
        
            fn into_system_generic(self) -> Box<dyn System> {
                Box::new(SystemGeneric::<_> {
                    system_with_marker: Box::new(self)
                })
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