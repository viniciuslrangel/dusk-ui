use twilight_model::channel::message;
use twilight_model::channel::message::component::ActionRow;

use crate::component::Component;
use crate::context::ContextPrefix;

pub struct Row<D> {
    phantom: std::marker::PhantomData<D>,
    pub children: Vec<Box<dyn Component<D>>>,
}

impl<D> Row<D> {
    pub fn new() -> Self {
        Self {
            phantom: Default::default(),
            children: Vec::new(),
        }
    }

    pub fn add<C: Component<D> + 'static>(mut self, child: C) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl<D> Row<D> {
    pub(crate) fn build(self, ctx: ContextPrefix<D>) -> message::Component {
        message::Component::ActionRow(ActionRow {
            components: self
                .children
                .into_iter()
                .enumerate()
                .map(|(i, x)| x.build(ctx.sub(i)))
                .collect(),
        })
    }
}

impl<D> Into<Option<Vec<Row<D>>>> for Row<D> {
    fn into(self) -> Option<Vec<Row<D>>> {
        Some(vec![self])
    }
}

impl<D, C: Component<D> + 'static> std::ops::MulAssign<C> for Row<D> {
    fn mul_assign(&mut self, rhs: C) {
        self.children.push(Box::new(rhs));
    }
}

impl<D, C: Component<D> + 'static> std::ops::Mul<C> for Row<D> {
    type Output = Self;

    fn mul(mut self, rhs: C) -> Self {
        self.children.push(Box::new(rhs));
        self
    }
}

impl<D> std::ops::Add<Row<D>> for Row<D> {
    type Output = Vec<Row<D>>;

    fn add(self, rhs: Row<D>) -> Self::Output {
        vec![self, rhs]
    }
}

impl<D> std::ops::Add<Row<D>> for Vec<Row<D>> {
    type Output = Vec<Row<D>>;

    fn add(mut self, rhs: Row<D>) -> Self::Output {
        self.push(rhs);
        self
    }
}
