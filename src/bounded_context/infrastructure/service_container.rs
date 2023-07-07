use std::any::TypeId;
use std::any::Any;
use std::collections::HashMap;

pub struct ServiceContainer {
    pub defined: bool,
    services: HashMap<TypeId, Box<dyn Any>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        ServiceContainer {
            defined: true,
            services: HashMap::new(),
        }        
    }

    pub fn register<T>(&mut self)
    where
        T: 'static + Default,
    {
        self.services.insert(TypeId::of::<T>(), Box::new(T::default()));
    }    

    pub fn is_registered<T>(&self) -> bool
    where
        T: 'static,
    {
        self.services.contains_key(&TypeId::of::<T>())
    }
}

#[cfg(test)]
mod tests {        
    use super::ServiceContainer;

    #[test]
    fn test_service_container_created() {
        let container = ServiceContainer::new();
        assert!(container.defined);
    }

    #[test]
    fn test_service_container_has_method_to_register_services() {
        let mut container = ServiceContainer::new();
        assert!(container.defined);

        // Register a service
        container.register::<MyService>();

        // Verify that the service is registered
        assert!(container.is_registered::<MyService>());
    }

    struct MyService {}

    impl Default for MyService {
        fn default() -> Self {
            MyService {}
        }
    }
}
