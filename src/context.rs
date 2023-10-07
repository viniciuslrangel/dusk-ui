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
            &Arc<Context<D>>,
            D,
        ) -> Pin<Box<dyn Future<Output = D> + Send + Sync>>
        + Send
        + Sync,
>;

pub struct Context<D> {
    pub(crate) should_exit: Mutex<bool>,
    pub(crate) dont_update: Mutex<bool>,
    phantom: std::marker::PhantomData<D>,
}

pub struct BuildContext<D> {
    pub(crate) binding: DashMap<String, Callback<D>>,
    pub(crate) ctx: Arc<Context<D>>,
}

pub struct BuildContextPrefix<'a, D> {
    pub parent: &'a BuildContext<D>,
    pub prefix: String,
}

impl<D> Context<D> {
    pub fn new() -> Self {
        Self {
            should_exit: Mutex::new(false),
            dont_update: Mutex::new(false),
            phantom: Default::default(),
        }
    }

    pub fn finish(&self) {
        *self.should_exit.lock().unwrap().deref_mut() = true;
    }

    pub fn dont_update(&self) {
        *self.dont_update.lock().unwrap().deref_mut() = true;
    }
}

impl<D> BuildContext<D> {
    pub fn new() -> Self {
        Self {
            binding: DashMap::new(),
            ctx: Arc::new(Context::new()),
        }
    }
}

impl<'a, D> BuildContextPrefix<'a, D> {
    pub fn sub<T: Display>(&self, prefix: T) -> Self {
        Self {
            parent: self.parent,
            prefix: format!("{}:{}", self.prefix, prefix),
        }
    }
}
