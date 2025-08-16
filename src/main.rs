use crate::{
    types::types::{Data, Error},
    utils::log::log,
};
use anyhow::Result;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use serenity::Client;

use std::env;

mod commands;
mod events;
mod types;
mod utils;

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            events::awake::awake(ctx, event, _framework, data_about_bot).await?;
        }
        // me when the
        serenity::FullEvent::Message { new_message } => {
            events::message::message(ctx, event, _framework, new_message).await?;
        }
        _ => {}
    };

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected discord token env");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::logs::logs(),
                commands::realtime::realtime(),
                commands::add::open::open(),
                commands::add::close::close(),
                commands::add::expire::expire(),
                commands::add::assign::assign(),
                commands::add::split::split(),
                commands::add::roll::roll(),
                commands::edit::edit::edit(),
                commands::edit::date::date(),
                commands::view::view::view(),
                commands::view::all::all(),
                commands::view::details::details(),
                commands::view::assets::assets(),
                commands::view::export::export(),
                commands::query::stats::stats(),
                commands::query::best::best(),
                commands::query::month::month(),
            ],
            event_handler: |ctx, event, framework, _data| {
                Box::pin(event_handler(ctx, event, framework))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = Client::builder(token, intents)
        //.event_handler(Handler {})
        .framework(framework)
        .await
        .expect("Could not create client");

    if let Err(e) = client.start().await.map_err(anyhow::Error::from) {
        let _ = log(format!("Client error: {}", e.to_string()));
        return Err(e);
    }
    Ok(())
}
