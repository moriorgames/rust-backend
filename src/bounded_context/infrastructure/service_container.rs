use crate::bounded_context::infrastructure::config::app_config::AppConfig;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;

pub struct ServiceContainer {
    services: HashMap<TypeId, Box<dyn Any>>,
}

pub trait Registrable {
    fn new_with_config(config: AppConfig) -> Self;
}

impl ServiceContainer {
    pub fn new() -> Self {
        ServiceContainer {
            services: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, app_config: AppConfig)
    where
        T: 'static + Registrable,
    {
        let service = T::new_with_config(app_config);
        self.services.insert(TypeId::of::<T>(), Box::new(service));
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
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_context::infrastructure::config::app_config::AppConfig;

    fn setup() -> ServiceContainer {
        ServiceContainer::new()
    }

    #[test]
    fn test_service_container_has_method_to_register_services() {
        let mut sut = setup();
        let config = AppConfig::default();

        // Register a service
        sut.register::<MyService>(config);

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
        let config = AppConfig::default();

        sut.register::<MyService>(config);

        let service: Option<&MyService> = sut.get();

        assert!(service.is_some());
    }

    #[allow(dead_code)]
    struct MyService {
        config: AppConfig,
    }

    impl Registrable for MyService {
        fn new_with_config(config: AppConfig) -> Self {
            MyService { config }
        }
    }
}
