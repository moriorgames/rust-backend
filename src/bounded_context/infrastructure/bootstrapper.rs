// use super::service_container::ServiceContainer;
use crate::bounded_context::infrastructure::config::app_config::AppConfig;

pub struct Bootstrapper {
    pub db_url: String
}

impl Bootstrapper {
    pub fn new() -> Self {
        let app_config = AppConfig::default();
        let db_url = app_config.db_url.clone();

        Self {
            db_url
        }
    }

    pub fn bootstrap(&self) -> &Self {
        self
    }
}

// pub struct Bootstrapper {
//     service_container: ServiceContainer
// }

// impl Bootstrapper {
//     pub fn new() -> Self {
//         Self {
//             service_container: ServiceContainer::new()
//         }
//     }

//     pub fn bootstrap(&mut self) {
//         self.service_container.register::<AppConfig>();
//     }
// }
