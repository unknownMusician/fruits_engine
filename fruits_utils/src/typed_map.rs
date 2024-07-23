use std::any::{Any, TypeId};
use std::collections::HashMap;

use strategies::TypedMapStrategy;

pub mod strategies {
    use std::any::Any;

    trait Sealed { }

    impl Sealed for DefaultStrategy { }
    impl Sealed for ThreadedStrategy { }

    #[allow(private_bounds)]
    pub trait TypedMapStrategy : Sealed {
        type ValueBase;

        fn downcast_ref<R: 'static>(stored: &Self::ValueBase) -> Option<&R>;
        fn downcast_mut<R: 'static>(stored: &mut Self::ValueBase) -> Option<&mut R>;
    }

    pub struct DefaultStrategy;
    impl TypedMapStrategy for DefaultStrategy {
        type ValueBase = Box<dyn Any>;

        fn downcast_ref<R: 'static>(stored: &Self::ValueBase) -> Option<&R> {
            stored.downcast_ref::<R>()
        }

        fn downcast_mut<R: 'static>(stored: &mut Self::ValueBase) -> Option<&mut R> {
            stored.downcast_mut::<R>()
        }
    }

    pub struct ThreadedStrategy;
    impl TypedMapStrategy for ThreadedStrategy {
        type ValueBase = Box<dyn Any + Send + Sync>;

        fn downcast_ref<R: 'static>(stored: &Self::ValueBase) -> Option<&R> {
            stored.downcast_ref::<R>()
        }

        fn downcast_mut<R: 'static>(stored: &mut Self::ValueBase) -> Option<&mut R> {
            stored.downcast_mut::<R>()
        }
    }
}

pub struct TypedMap<Strategy: TypedMapStrategy = strategies::DefaultStrategy> {
    data: HashMap<TypeId, Strategy::ValueBase>,
}

impl<Strategy: TypedMapStrategy> TypedMap<Strategy> {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn get_ref<T: 'static + Any>(&self) -> Option<&T> {
        self.data
            .get(&TypeId::of::<T>())
            .map(|b| Strategy::downcast_ref::<T>(b).unwrap())
    }

    pub fn get_mut<T: 'static + Any>(&mut self) -> Option<&mut T> {
        self.data
            .get_mut(&TypeId::of::<T>())
            .map(|b| Strategy::downcast_mut::<T>(b).unwrap())
    }
}

impl TypedMap<strategies::DefaultStrategy> {
    pub fn insert<T: 'static + Any>(&mut self, v: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(v));
    }
}

impl TypedMap<strategies::ThreadedStrategy> {
    pub fn insert<T: 'static + Any + Send + Sync>(&mut self, v: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(v));
    }
}