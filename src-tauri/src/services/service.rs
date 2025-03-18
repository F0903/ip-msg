use std::any::{Any, TypeId};

pub trait Service {}

pub trait ServiceId {
    const SERVICE_ID: TypeId;
}

// Automatically expose SERVICE_NAME (which is the type name) for any type that implements ServiceInterface
impl<T: ServiceInterface> ServiceId for T {
    const SERVICE_ID: TypeId = TypeId::of::<T>();
}

pub trait ServiceInterface: Any {
    fn as_any(&self) -> &dyn Any;
    fn get_service_id(&self) -> TypeId;
}

impl<T: Service + 'static> ServiceInterface for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_service_id(&self) -> TypeId {
        Self::SERVICE_ID
    }
}
