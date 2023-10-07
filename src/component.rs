use twilight_model::channel::message;

use crate::context::BuildContextPrefix;

pub mod button;
pub mod row_button;
pub mod select_menu;

pub trait Component<D> {
    fn build(self: Box<Self>, ctx: BuildContextPrefix<D>) -> message::Component;
}

#[derive(Default)]
pub struct CompWindow<D> {
    phantom: std::marker::PhantomData<D>,
    pub(crate) children: Vec<Box<dyn Component<D>>>,
}

impl<D> CompWindow<D> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
            children: Vec::new(),
        }
    }
}

impl<D, C: 'static + Component<D>> std::ops::Add<C> for CompWindow<D> {
    type Output = Self;

    fn add(mut self, rhs: C) -> Self::Output {
        self.children.push(Box::new(rhs));
        self
    }
}
