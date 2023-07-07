pub struct ServiceContainer {
    pub defined: bool
}

impl ServiceContainer {
    pub fn new() -> Self {
        ServiceContainer {
            defined: true
        }        
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
}
