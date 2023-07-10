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

    pub fn get<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        self.services.get(&TypeId::of::<T>()).and_then(|boxed| boxed.downcast_ref())
    }

}

#[cfg(test)]
mod tests {
    use super::ServiceContainer;

    fn setup() -> ServiceContainer {
        ServiceContainer::new()
    }

    #[test]
    fn test_service_container_created() {
        let sut = setup();
        assert!(sut.defined);
    }

    #[test]
    fn test_service_container_has_method_to_register_services() {
        let mut sut = setup();
        assert!(sut.defined);

        // Register a service
        sut.register::<MyService>();

        // Verify that the service is registered
        assert!(sut.is_registered::<MyService>());
    }

    #[test]
    fn test_service_container_get_service() {
        let sut = setup();
        let service: Option<&MyService> = sut.get();

        assert!(service.is_none());
    }

    #[test]
    fn test_service_container_get_registered_service() {
        let mut sut = setup();
        sut.register::<MyService>();
    
        let service: Option<&MyService> = sut.get();
    
        assert!(service.is_some());
    }
    
    struct MyService {}

    impl Default for MyService {
        fn default() -> Self {
            MyService {}
        }
    }
}
