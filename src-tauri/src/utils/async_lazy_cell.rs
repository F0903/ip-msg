use std::{cell::UnsafeCell, future::Future, pin::Pin};

type Result<T, E> = std::result::Result<T, E>;

enum State<T, F> {
    Init(T),
    Uninit(F),
}

pub struct AsyncLazyCell<T, E> {
    state: UnsafeCell<State<T, fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>>>,
}

impl<T, E> AsyncLazyCell<T, E> {
    pub const fn new(getter: fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>) -> Self {
        Self {
            state: UnsafeCell::new(State::Uninit(getter)),
        }
    }

    pub const fn with_value(value: T) -> Self {
        Self {
            state: UnsafeCell::new(State::Init(value)),
        }
    }

    async fn init(&self) -> Result<&T, E> {
        let state = unsafe { &*self.state.get() };
        match state {
            State::Uninit(f) => {
                let val = f().await?;

                let state_mut = unsafe { &mut *self.state.get() };
                *state_mut = State::Init(val);

                let val_ref = unsafe { &*self.state.get() };
                match val_ref {
                    State::Init(x) => Ok(x),
                    State::Uninit(_) => unreachable!(),
                }
            }
            State::Init(_) => unreachable!(),
        }
    }

    pub async fn get<'a>(&'a self) -> Result<&'a T, E>
    where
        T: 'a,
    {
        let state = unsafe { &*self.state.get() };
        match state {
            State::Init(x) => Ok(x),
            State::Uninit(_) => {
                self.init().await?;
                let new_state = unsafe { &*self.state.get() };
                match new_state {
                    State::Init(x) => Ok(x),
                    _ => unreachable!(),
                }
            }
        }
    }
}

unsafe impl<T: Send, E: Send> Send for AsyncLazyCell<T, E> {}
unsafe impl<T: Sync, E: Sync> Sync for AsyncLazyCell<T, E> {}

// Can't be used as planned due to https://github.com/rust-lang/rust/issues/100013
/* pub trait ToInnerRef<T, E> {
    async fn to_inner_ref<'a>(&'a self) -> Result<&'a T, E>
    where
        T: 'a;
}

impl<T, E> ToInnerRef<T, E> for AsyncLazyCell<T, E> {
    async fn to_inner_ref<'a>(&'a self) -> Result<&'a T, E>
    where
        T: 'a,
    {
        let val = self.get().await;
        val
    }
} */
