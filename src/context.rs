use std::fmt::Display;
use std::future::Future;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use twilight_model::gateway::payload::incoming::InteractionCreate;

pub type Callback<D> = Box<
    dyn Fn(
            &Box<InteractionCreate>,
            &Context<D>,
            D,
        ) -> Pin<Box<dyn Future<Output = D> + Send + Sync>>
        + Send
        + Sync,
>;

pub struct Context<D> {
    pub(crate) binding: DashMap<String, Callback<D>>,
    pub(crate) should_exit: Arc<Mutex<bool>>,
}

pub struct ContextPrefix<'a, D> {
    pub parent: &'a Context<D>,
    pub prefix: String,
}

impl<D> Context<D> {
    pub fn new() -> Self {
        Self {
            binding: DashMap::new(),
            should_exit: Arc::new(Mutex::new(false)),
        }
    }

    pub fn finish(&self) {
        *self.should_exit.lock().unwrap().deref_mut() = true;
    }
}

impl<'a, D> ContextPrefix<'a, D> {
    pub fn sub<T: Display>(&self, prefix: T) -> Self {
        Self {
            parent: self.parent,
            prefix: format!("{}:{}", self.prefix, prefix),
        }
    }
}
