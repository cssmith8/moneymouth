use crate::{
    types::types::{AppContext, Error},
    utils::{
        date::date_from_string,
        db::{get_options_db_path, get_selected_position, open_options_db, position_list_replace},
        log::log,
    },
};
use chrono::prelude::*;
//use poise::serenity_prelude::CreateQuickModal;
use anyhow::Result;
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Edit Position"] // Struct name by default
struct EditModal {
    #[name = "Stock Ticker"] // Field name by default
    #[placeholder = "AMZN"] // No placeholder by default
    #[min_length = 2] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 16]
    //#[paragraph] // Switches from single-line input to multiline text box
    ticker: Option<String>,
    #[name = "Strike Price"]
    #[placeholder = "10.00"]
    strike: Option<String>,
    #[name = "Expiration Date"]
    #[placeholder = "12/30/2025"]
    #[max_length = 10]
    exp: Option<String>,
    #[name = "Premium"]
    #[placeholder = "0.50"]
    premium: Option<String>,
    #[name = "Quantity"]
    #[placeholder = "1"]
    quantity: Option<String>,
}

/// Edit details of the selected options position
#[poise::command(slash_command)]
pub async fn edit(ctx: AppContext<'_>) -> Result<(), Error> {
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
            let _ = log(format!("Error: {}", err));
            return Ok(());
        }
    };
    let mut position = indexed_position.position;

    let last_index = position.contracts.len() - 1;

    //execute the modal
    let data = match EditModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    if let Some(ticker) = data.ticker {
        position.contracts[last_index].open.ticker = ticker;
    }
    if let Some(strike) = data.strike {
        position.contracts[last_index].open.strike = strike.parse::<f64>()?;
    }
    if let Some(exp) = data.exp {
        let expiry_string = date_from_string(exp)?;
        position.contracts[last_index].open.expiry = match Utc.with_ymd_and_hms(
            expiry_string.year,
            expiry_string.month,
            expiry_string.day,
            20,
            0,
            0,
        ) {
            chrono::LocalResult::Single(datetime) => datetime,
            _ => return Err(Error::from("Invalid date provided")),
        };
    }
    if let Some(premium) = data.premium {
        position.contracts[last_index].open.premium = premium.parse::<f64>()?;
    }
    if let Some(quantity) = data.quantity {
        position.contracts[last_index].open.quantity = quantity.parse::<u16>()?;
    }
    position_list_replace(
        &mut db,
        "positions",
        indexed_position.index as usize,
        position,
    );

    ctx.say("Position Updated").await?;
    db.set("edit_id", &-1)?;
    Ok(())
}
