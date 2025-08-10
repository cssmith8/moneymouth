use crate::types::option::OptionClose;
use crate::types::types::{AppContext, Error};
use crate::utils::db::{
    get_options_db_path, get_selected_position, open_options_db, position_list_replace,
};
use anyhow::Result;
use chrono::prelude::*;
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Close Contract"] // Struct name by default
pub struct CloseModal {
    #[name = "Price"]
    #[placeholder = "0.10"]
    premium: String,
    //#[name = "Quantity"]
    //#[placeholder = "1"]
    //quantity: String,
}

#[poise::command(slash_command)]
pub async fn close(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = get_options_db_path(userid.to_string());

    let mut db = match open_options_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };
    let indexed_position = match get_selected_position(&db) {
        Ok(pos) => pos,
        Err(err) => {
            ctx.say("An error has occurred").await?;
            println!("Error: {}", err);
            return Ok(());
        }
    };
    let mut position = indexed_position.position;

    //execute the modal
    let data = match CloseModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    let last_index = position.contracts.len() - 1;
    position.contracts[last_index].close = Some(OptionClose {
        date: Utc::now(),
        close_type: "close".to_string(),
        premium: data.premium.parse::<f64>()?,
    });
    position.contracts[last_index].open.status = "closed".to_string();
    let gain: f64 = position.gain();

    position_list_replace(
        &mut db,
        "positions",
        indexed_position.index as usize,
        position,
    );

    let money_mouth = if gain > 0.0 { ":money_mouth:" } else { "" };
    ctx.say(format!(
        "Contract Closed. You made ${:.2} {}",
        gain, money_mouth
    ))
    .await?;

    db.set("edit_id", &-1)?;
    Ok(())
}
