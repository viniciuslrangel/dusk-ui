use twilight_model::channel::message;

use crate::context::ContextPrefix;

pub mod button;
pub mod row;
pub mod row_button;

pub trait Component<D> {
    fn build(self: Box<Self>, ctx: ContextPrefix<D>) -> message::Component;
}

pub trait RootComponent<D>: Component<D> {}

#[derive(Default)]
pub struct CompWindow<D> {
    phantom: std::marker::PhantomData<D>,
    pub(crate) children: Vec<Box<dyn RootComponent<D>>>,
}

impl<D> CompWindow<D> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
            children: Vec::new(),
        }
    }
}

impl<D, C: 'static + RootComponent<D>> std::ops::Add<C> for CompWindow<D> {
    type Output = Self;

    fn add(mut self, rhs: C) -> Self::Output {
        self.children.push(Box::new(rhs));
        self
    }
}
