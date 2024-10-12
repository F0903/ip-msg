use std::{future::Future, pin::Pin};

type Result<T, E> = std::result::Result<T, E>;

enum State<T, F> {
    Init(T),
    Uninit(F),
}

pub struct AsyncLazyCell<T, E> {
    state: State<T, fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>>,
}

impl<T, E> AsyncLazyCell<T, E> {
    pub const fn new(getter: fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>) -> Self {
        Self {
            state: State::Uninit(getter),
        }
    }

    pub const fn with_value(value: T) -> Self {
        Self {
            state: State::Init(value),
        }
    }

    fn init(&self) {
        //TODO
    }

    pub async fn get<'a>(&'a self) -> Result<&'a T, E>
    where
        T: 'a,
    {
        match &self.state {
            State::Init(x) => Ok(x),
            State::Uninit(_) => {
                self.init();
                Ok(match &self.state {
                    State::Init(x) => x,
                    _ => unreachable!(),
                })
            }
        }
    }
}

unsafe impl<T: Send, E: Send> Send for AsyncLazyCell<T, E> {}
unsafe impl<T: Sync, E: Sync> Sync for AsyncLazyCell<T, E> {}
