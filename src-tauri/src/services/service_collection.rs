use super::{ServiceInterface, ServiceName};
use std::{collections::HashMap, sync::Arc};

pub trait ThreadSafeServiceInterface = ServiceInterface + Send + Sync;

pub struct ServiceCollection {
    services: HashMap<&'static str, Arc<dyn ThreadSafeServiceInterface>>,
}

impl ServiceCollection {
    pub fn with_services(services: Vec<Arc<dyn ThreadSafeServiceInterface>>) -> Self {
        let service_map = services
            .into_iter()
            .map(|x| (x.get_service_name(), x))
            .collect();
        Self {
            services: service_map,
        }
    }

    pub fn get_service<T: ThreadSafeServiceInterface + ServiceName>(&self) -> &T {
        self.services
            .get(T::SERVICE_NAME)
            .and_then(|service| service.as_any().downcast_ref::<T>())
            .expect("Requested service was not found in ServiceCollection")
    }
}
