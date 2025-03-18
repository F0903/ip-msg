use super::{ServiceId, ServiceInterface};
use std::{any::TypeId, collections::HashMap, sync::Arc};

pub trait ThreadSafeServiceInterface = ServiceInterface + Send + Sync;

pub struct ServiceCollection {
    services: HashMap<TypeId, Arc<dyn ThreadSafeServiceInterface>>,
}

impl ServiceCollection {
    pub fn with_services(services: Vec<Arc<dyn ThreadSafeServiceInterface>>) -> Self {
        let service_map = services
            .into_iter()
            .map(|x| (x.get_service_id(), x))
            .collect();
        Self {
            services: service_map,
        }
    }

    pub fn get_service<T: ThreadSafeServiceInterface + ServiceId>(&self) -> &T {
        self.services
            .get(&T::SERVICE_ID)
            .and_then(|service| service.as_any().downcast_ref::<T>())
            .expect("Requested service was not found in ServiceCollection")
    }
}
