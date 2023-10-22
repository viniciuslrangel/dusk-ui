use std::sync::Arc;

use crate::errors;
use dashmap::DashMap;
use tokio::sync::oneshot::Sender;
use twilight_http::client::InteractionClient;
use twilight_model::application::interaction::InteractionType;
use twilight_model::gateway::event::Event;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_model::id::marker::MessageMarker;
use twilight_model::id::Id;

pub struct ProcessResult {
    pub processed: bool,
}

#[derive(Debug, Default)]
pub struct Dusk {
    pub(crate) messages: DashMap<Id<MessageMarker>, Sender<Box<InteractionCreate>>>,
}

impl Dusk {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            messages: DashMap::new(),
        })
    }

    pub async fn process<'a>(
        &self,
        event: &Event,
        interaction_client: &InteractionClient<'a>,
    ) -> errors::Result<ProcessResult> {
        match event {
            Event::InteractionCreate(e) => {
                if e.kind == InteractionType::MessageComponent {
                    if let Some(message) = &e.message {
                        if let Some((_, sender)) = self.messages.remove(&message.id) {
                            interaction_client
                                .create_response(
                                    e.id,
                                    &e.token,
                                    &InteractionResponse {
                                        kind: InteractionResponseType::DeferredUpdateMessage,
                                        data: None,
                                    },
                                )
                                .await?;
                            let payload = e.clone();
                            let _ = sender.send(payload);
                            return Ok(ProcessResult { processed: true });
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(ProcessResult { processed: false })
    }
}
