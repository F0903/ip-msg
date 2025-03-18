use std::any::Any;

pub trait Service {}

pub trait ServiceName {
    const SERVICE_NAME: &'static str;
}

// Automatically expose SERVICE_NAME (which is the type name) for any type that implements ServiceInterface
impl<T: ServiceInterface> ServiceName for T {
    const SERVICE_NAME: &'static str = std::any::type_name::<T>();
}

pub trait ServiceInterface: Any {
    fn as_any(&self) -> &dyn Any;
    fn get_service_name(&self) -> &'static str;
}

impl<T: Service + 'static> ServiceInterface for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_service_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}
