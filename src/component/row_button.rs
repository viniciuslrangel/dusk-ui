use std::mem::MaybeUninit;

use twilight_model::channel::message;
use twilight_model::channel::message::component::ActionRow;

use crate::component::button::Button;
use crate::component::Component;
use crate::context::BuildContextPrefix;

pub struct RowButton<D, const N: usize = 0> {
    phantom: std::marker::PhantomData<D>,
    children: [Button<D>; N],
}

impl<D> RowButton<D, 0> {
    pub fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
            children: [],
        }
    }
}

macro_rules! row_impl {
    ($count: literal) => {
        impl<D> RowButton<D, $count> {
            pub(crate) fn push(self, child: Button<D>) -> RowButton<D, { $count + 1 }> {
                let mut data: [MaybeUninit<Button<D>>; { $count + 1 }] =
                    unsafe { MaybeUninit::uninit().assume_init() };
                let mut i = 0;
                for x in self.children {
                    data[i].write(x);
                    i += 1;
                }
                data[i].write(child);
                RowButton {
                    phantom: Default::default(),
                    children: data.map(|x| unsafe { x.assume_init() }),
                }
            }
        }

        impl<D> std::ops::Mul<Button<D>> for RowButton<D, $count> {
            type Output = RowButton<D, { $count + 1 }>;

            fn mul(self, rhs: Button<D>) -> Self::Output {
                self.push(rhs)
            }
        }
    };
}

row_impl!(0);
row_impl!(1);
row_impl!(2);
row_impl!(3);
row_impl!(4);

impl<D, const N: usize> Component<D> for RowButton<D, N> {
    fn build(self: Box<Self>, ctx: BuildContextPrefix<D>) -> message::Component {
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
