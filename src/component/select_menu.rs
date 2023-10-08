use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use twilight_model::channel::message;
use twilight_model::channel::message::component::SelectMenuOption;
use twilight_model::channel::message::ReactionType;
use twilight_model::gateway::payload::incoming::InteractionCreate;

use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::component::Component;
use crate::context::{BuildContextPrefix, Callback, Context};

pub struct SelectMenu<D> {
    phantom: std::marker::PhantomData<D>,
    pub id: String,
    pub disabled: bool,
    pub max_values: Option<u8>,
    pub min_values: Option<u8>,
    pub options: Vec<SelectMenuOption>,
    pub placeholder: Option<String>,
    pub on_change: Option<Callback<D>>,
}

impl<D> SelectMenu<D> {
    pub fn new() -> Self {
        Self {
            id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect(),
            ..Default::default()
        }
    }

    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = id.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn max_values(mut self, max_values: u8) -> Self {
        self.max_values = Some(max_values);
        self
    }

    pub fn min_values(mut self, min_values: u8) -> Self {
        self.min_values = Some(min_values);
        self
    }

    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options.into_iter().map(|e| e.into()).collect();
        self
    }

    pub fn placeholder<S: Into<String>>(mut self, placeholder: S) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn on_change<
        F: 'static
            + Fn(
                &Box<InteractionCreate>,
                &Arc<Context<D>>,
                D,
            ) -> Pin<Box<dyn Future<Output = D> + Send>>
            + Send
            + Sync,
    >(
        mut self,
        f: F,
    ) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }
}

impl<D> Default for SelectMenu<D> {
    fn default() -> Self {
        Self {
            phantom: std::marker::PhantomData,
            id: String::new(),
            disabled: false,
            max_values: None,
            min_values: None,
            options: Vec::new(),
            placeholder: None,
            on_change: None,
        }
    }
}

impl<D> Component<D> for SelectMenu<D> {
    fn build(
        mut self: Box<Self>,
        ctx: BuildContextPrefix<D>,
    ) -> twilight_model::channel::message::Component {
        let id = format!("{}.{}", ctx.prefix, self.id);
        if let Some(on_change) = self.on_change.take() {
            ctx.parent.binding.insert(id.clone(), on_change);
        }
        let comp = message::Component::SelectMenu(message::component::SelectMenu {
            custom_id: id,
            disabled: self.disabled,
            max_values: self.max_values,
            min_values: self.min_values,
            options: self.options,
            placeholder: self.placeholder,
        });
        message::Component::ActionRow(message::component::ActionRow {
            components: vec![comp],
        })
    }
}

pub struct SelectOption {
    pub default: bool,
    pub description: Option<String>,
    pub emoji: Option<ReactionType>,
    pub label: String,
    pub value: String,
}

impl Default for SelectOption {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectOption {
    pub fn new() -> Self {
        Self {
            default: false,
            description: None,
            emoji: None,
            label: String::new(),
            value: String::new(),
        }
    }

    pub fn is_default(mut self, default: bool) -> Self {
        self.default = default;
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn emoji(mut self, emoji: ReactionType) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = label.into();
        self
    }

    pub fn value<S: Into<String>>(mut self, value: S) -> Self {
        self.value = value.into();
        self
    }
}

impl Into<message::component::SelectMenuOption> for SelectOption {
    fn into(self) -> message::component::SelectMenuOption {
        message::component::SelectMenuOption {
            default: self.default,
            description: self.description,
            emoji: self.emoji,
            label: self.label,
            value: self.value,
        }
    }
}
