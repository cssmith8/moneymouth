use crate::{
    types::types::{Data, Error},
    utils::log::log,
};
use anyhow::Result;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use std::env;

pub async fn awake(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data_about_bot: &serenity::Ready,
) -> Result<(), Error> {
    let _ = log(format!("Logged in as {}", data_about_bot.user.tag()));

    rustical_message(
        _ctx,
        ChannelId::new(1160065321013620857), //bot
        //ChannelId::new(1120455140416172115), //genny
        env::var("LAPTOP").expect("0"),
    )
    .await?;

    Ok(())
}

async fn rustical_message(
    ctx: &serenity::Context,
    c: ChannelId,
    laptop: String,
) -> Result<(), Error> {
    let message: String = match laptop.parse().unwrap() {
        2 => "Fiddlesticks Dockerically :face_with_monocle:".to_string(),
        1 => "Fiddlesticks Laptopically :money_mouth:".to_string(),
        _ => "Fiddlesticks :money_mouth:".to_string(),
    };

    let channel = c;
    let channel = channel
        .to_channel(&ctx.http)
        .await
        .expect("this channel will always work");
    if let Some(channel) = channel.guild() {
        channel.say(&ctx.http, &message).await?;
    }
    Ok(())
}
