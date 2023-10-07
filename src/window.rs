use tokio::sync::oneshot;
use twilight_http::client::InteractionClient;
use twilight_model::application::interaction::{Interaction, InteractionData};
use twilight_model::channel::message::MessageFlags;
use twilight_model::channel::{message, Message};
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

use crate::component::CompWindow;
use crate::context::{Context, ContextPrefix};
use crate::dusk::Dusk;

fn draw_text<D, FT, FTR>(data: &D, render_text: &FT) -> Option<String>
where
    FT: Fn(&D) -> FTR + 'static,
    FTR: Into<Option<String>> + 'static,
{
    render_text(data).into()
}

fn draw_components<D, FC>(
    ctx: &Context<D>,
    data: &D,
    render_cmp: &FC,
) -> Option<Vec<message::Component>>
where
    FC: Fn(&D) -> CompWindow<D>,
{
    let comps = render_cmp(data);
    let vec = comps
        .children
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            x.build(ContextPrefix {
                parent: ctx,
                prefix: i.to_string(),
            })
        })
        .collect();
    Some(vec)
}

pub async fn create<'a, D, FT, FTR, FC>(
    dusk: &Dusk,
    interaction: &'a Interaction,
    client: &'a InteractionClient<'a>,
    ephemeral: bool,
    no_defer: bool,
    mut data: D,
    render_text: FT,
    render_cmp: FC,
) -> crate::errors::Result<()>
where
    FT: Fn(&D) -> FTR + 'static,
    FTR: Into<Option<String>> + 'static,
    FC: Fn(&D) -> CompWindow<D>,
{
    let last_text = "".to_string();

    let ctx = Context::new();

    if !no_defer {
        client
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::DeferredChannelMessageWithSource,
                    data: if ephemeral {
                        Some(InteractionResponseData {
                            flags: Some(MessageFlags::EPHEMERAL),
                            ..Default::default()
                        })
                    } else {
                        None
                    },
                },
            )
            .await?;
    }

    let mut msg: Option<Message> = None;
    let token: String = interaction.token.clone();
    while !*ctx.should_exit.lock().unwrap() {
        if let Some(msg) = msg.as_ref() {
            dusk.messages.remove(&msg.id);
        }

        let mut update = client.update_response(&token);

        let new_text = draw_text(&data, &render_text);
        if let Some(new_text) = &new_text {
            if *new_text != last_text {
                update = update.content(Some(new_text.as_str()))?;
            }
        }

        ctx.binding.clear();
        let components = draw_components(&ctx, &data, &render_cmp);
        update = update.components(components.as_deref())?;

        let result = update.await?.model().await?;
        msg = Some(result);

        let (tx, rx) = oneshot::channel();
        dusk.messages.insert(msg.as_ref().unwrap().id, tx);

        let interaction = rx.await?;
        if let Some(InteractionData::MessageComponent(interaction_data)) = &interaction.data {
            let custom_id = &interaction_data.custom_id;
            if let Some(callback) = ctx.binding.get(custom_id) {
                let callback = callback.value();
                data = callback(&interaction, &ctx, data).await;
            }
        }
    }

    Ok(())
}
