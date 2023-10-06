use std::future::Future;
use std::pin::Pin;

use twilight_model::channel::message;
use twilight_model::channel::message::component::ButtonStyle;
use twilight_model::channel::message::ReactionType;
use twilight_model::gateway::payload::incoming::InteractionCreate;

use crate::component::Component;
use crate::context::{Context, ContextPrefix};

pub struct Button<D> {
    pub id: String,
    pub disabled: bool,
    pub emoji: Option<ReactionType>,
    pub label: Option<String>,
    pub style: ButtonStyle,
    pub url: Option<String>,
    pub on_click: Option<
        Box<
            dyn Fn(
                    &Box<InteractionCreate>,
                    &Context<D>,
                    D,
                ) -> Pin<Box<dyn Future<Output = D> + Send + Sync>>
                + Send
                + Sync,
        >,
    >,
}

impl<D> Button<D> {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            ..Default::default()
        }
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn with_emoji(mut self, emoji: ReactionType) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_url(mut self, url: String) -> Self {
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

impl<D> Component<D> for Button<D> {
    fn build(mut self: Box<Self>, ctx: ContextPrefix<D>) -> message::Component {
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
