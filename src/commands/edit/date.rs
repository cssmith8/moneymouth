use crate::{
    types::types::{AppContext, Error},
    utils::{
        db::{get_options_db_path, get_selected_position, open_options_db, position_list_replace},
        log::log,
    },
};
use anyhow::Result;
use chrono::prelude::*;
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Edit Position"] // Struct name by default
struct DateModal {
    #[name = "Year"] // Field name by default
    #[placeholder = "2025"] // No placeholder by default
    #[max_length = 4]
    //#[paragraph] // Switches from single-line input to multiline text box
    year: Option<String>,
    #[name = "Month"]
    #[placeholder = "12"]
    #[max_length = 2]
    month: Option<String>,
    #[name = "Day"]
    #[placeholder = "30"]
    #[max_length = 2]
    day: Option<String>,
}

/// Edit the start date of the selected options contract
#[poise::command(slash_command)]
pub async fn date(ctx: AppContext<'_>) -> Result<(), Error> {
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

    let data = match DateModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    // Extract current date fields before mutably borrowing contracts
    let (mut cur_year, mut cur_month, mut cur_day) = {
        let final_date = position.get_final_contract().open.date;
        (final_date.year(), final_date.month(), final_date.day())
    };

    let last_idx = position.contracts.len() - 1;

    // Track if any fields were updated
    let mut updated_fields = false;

    // Update the working values instead of the position directly
    if let Some(year) = data.year {
        cur_year = year.parse::<i32>()?;
        updated_fields = true;
    }

    if let Some(month) = data.month {
        cur_month = month.parse::<u32>()?;
        updated_fields = true;
    }

    if let Some(day) = data.day {
        cur_day = day.parse::<u32>()?;
        updated_fields = true;
    }

    // Apply all changes at once if any fields were updated
    if updated_fields {
        position.contracts[last_idx].open.date =
            match Utc.with_ymd_and_hms(cur_year, cur_month, cur_day, 17, 0, 0) {
                chrono::LocalResult::Single(datetime) => datetime,
                _ => return Err(Error::from("Invalid date provided")),
            };
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
