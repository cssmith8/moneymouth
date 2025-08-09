use crate::types::types::{Data, Error};
use anyhow::Result;
use poise::serenity_prelude as serenity;

pub async fn message(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    new_message: &serenity::Message,
) -> Result<(), Error> {
    if new_message.author.bot {
        return Ok(());
    }
    let channel_id = new_message.channel_id;
    let content = new_message.content.to_lowercase();

    match content.as_str() {
        "money mouth" => {
            let response = "I'm moneying my mouth";
            channel_id.say(&_ctx.http, response).await?;
            return Ok(());
        }
        _ => {}
    }
    Ok(())
}
