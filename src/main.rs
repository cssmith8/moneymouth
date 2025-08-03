//use csv::{Reader, StringRecord, Writer};
use crate::types::types::{Context, Data, Error};
use anyhow::Result;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use serenity::{
    //model::prelude::{Message, Ready},
    Client,
};

use std::env;

mod commands;
mod types;
mod utils;

#[poise::command(slash_command, prefix_command)]
async fn say(
    ctx: Context<'_>,
    #[description = "Message to say"] message: String,
) -> Result<(), Error> {
    ctx.say(message).await?;
    Ok(())
}

//send a message in channel c
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

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.tag());

            let bot_channel_id: u64 = env::var("BOT_CHANNEL_ID")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);

            match bot_channel_id {
                0 => println!("No bot channel ID set in .env"),
                _ => {
                    rustical_message(
                        ctx,
                        ChannelId::new(bot_channel_id),
                        env::var("LAPTOP").expect("0"),
                    )
                    .await?;
                }
            }
        }
        // me when the
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }
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
                say(),
                commands::options::add::open::open(),
                commands::options::add::close::close(),
                commands::options::add::expire::expire(),
                commands::options::add::assign::assign(),
                commands::options::add::split::split(),
                commands::options::add::roll::roll(),
                commands::options::edit::edit::edit(),
                commands::options::edit::date::date(),
                commands::options::view::view::view(),
                commands::options::view::all::all(),
                commands::options::view::details::details(),
                commands::options::view::assets::assets(),
                commands::options::query::stats::stats(),
                commands::options::query::best::best(),
                commands::options::query::month::month(),
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
        println!("Client error: {}", e.to_string());
        return Err(e);
    }
    Ok(())
}
