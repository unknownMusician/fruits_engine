use std::any::Any;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::typed_map::TypedMap;

pub struct Container {
    bindings: TypedMap,
}

impl Container {
    pub fn new() -> Self {
        Self {
            bindings: TypedMap::new()
        }
    }

    fn bind_fn<T: Send + Sync + Any + ?Sized + 'static>(&mut self, f: Box<dyn Send + Sync + Fn() -> Rc<T>>) {
        self.bindings.insert(f);
    }

    pub fn resolve<T: Send + Sync + Any + ?Sized + 'static>(&self) -> Option<Rc<T>> {
        self.bindings
            .get_ref::<Box<dyn Fn() -> Rc<T>>>()
            .map(|f| f())
    }
}

pub struct Binder<'a, T: Send + Sync + Any + ?Sized + 'static> {
    container: &'a mut Container,
    phantom: PhantomData<T>,
}

impl Container {
    pub fn bind<T: Send + Sync + Any + ?Sized + 'static>(&mut self) -> Binder<T> {
        Binder {
            container: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Send + Sync + Any + ?Sized + 'static> Binder<'a, T> {
    pub fn to_singleton(self, v: Rc<T>) {
        self.container.bind_fn(Box::new(move || Rc::clone(&v)));
    }

    pub fn to_transient(self, v: Box<dyn Fn() -> Rc<T>>) {
        self.container.bind_fn(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_container_resolves_none() {
        let container = Container::new();

        assert_eq!(container.resolve::<i32>(), None)
    }

    #[test]
    fn container_with_string_singleton_resolves_i32_as_none() {
        let mut container = Container::new();

        container.bind().to_singleton(Rc::new(String::from("text")));

        assert_eq!(container.resolve::<i32>(), None);
    }

    #[test]
    fn container_with_i32_singleton_resolves_bound_i32() {
        let mut container = Container::new();

        let number: i32 = 456;

        container.bind().to_singleton(Rc::new(number));

        assert_eq!(container.resolve::<i32>().map(|x| *x), Some(number));
    }
}

mod use_case {
    use std::rc::Rc;

    pub fn execute() {
        let mut container = crate::di::Container::new();

        container
            .bind::<dyn Logger>()
            .to_singleton(Rc::new(UppercaseLogger));

        match container.resolve::<dyn Logger>() {
            Some(logger) => logger.log("Hello from context!"),
            None => println!("Failed to resolve."),
        }
    }

    trait Logger: Send + Sync {
        fn log(&self, v: &str);
    }

    struct DefaultLogger;

    impl Logger for DefaultLogger {
        fn log(&self, v: &str) {
            println!("{}", v);
        }
    }

    struct UppercaseLogger;

    impl Logger for UppercaseLogger {
        fn log(&self, v: &str) {
            println!("{}", v.to_uppercase());
        }
    }
}