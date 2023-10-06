use twilight_model::channel::message;

use crate::context::ContextPrefix;

pub mod button;
pub mod row;

pub trait Component<D> {
    fn build(self: Box<Self>, ctx: ContextPrefix<D>) -> message::Component;
}
