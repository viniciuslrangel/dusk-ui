use std::future::Future;
use std::pin::Pin;

use rand::distributions::Alphanumeric;
use rand::Rng;
use twilight_model::channel::message;
use twilight_model::channel::message::component::ButtonStyle;
use twilight_model::channel::message::ReactionType;
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::context::{Callback, Context, ContextPrefix};

pub struct Button<D> {
    pub id: String,
    pub disabled: bool,
    pub emoji: Option<ReactionType>,
    pub label: Option<String>,
    pub style: ButtonStyle,
    pub url: Option<String>,
    pub on_click: Option<Callback<D>>,
}

impl<D> Button<D> {
    pub fn new_rand() -> Self {
        Self {
            id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect(),
            ..Default::default()
        }
    }

    pub fn new<S: Into<String>>(label: S) -> Self {
        let mut s = Self::new_rand();
        s.label = Some(label.into());
        s
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn emoji(mut self, emoji: ReactionType) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn on_click<
        F: 'static
            + Fn(
                &Box<InteractionCreate>,
                &Context<D>,
                D,
            ) -> Pin<Box<dyn Future<Output = D> + Send + Sync>>
            + Send
            + Sync,
    >(
        mut self,
        f: F,
    ) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl<D> Default for Button<D> {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            disabled: false,
            emoji: None,
            label: None,
            style: ButtonStyle::Primary,
            url: None,
            on_click: None,
        }
    }
}

impl<D> Button<D> {
    pub(crate) fn build(mut self: Self, ctx: ContextPrefix<D>) -> message::Component {
        let id = format!("{}.{}", ctx.prefix, self.id);
        if let Some(on_click) = self.on_click.take() {
            ctx.parent.binding.insert(id.clone(), on_click);
        }
        message::Component::Button(message::component::Button {
            custom_id: Some(id),
            disabled: self.disabled,
            emoji: self.emoji.clone(),
            label: self.label.clone(),
            style: self.style,
            url: self.url.clone(),
        })
    }
}
