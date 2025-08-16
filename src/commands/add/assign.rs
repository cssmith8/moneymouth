use crate::{
    types::types::{AppContext, Error},
    utils::db::{
        get_options_db_path, get_selected_position, open_options_db, position_list_replace,
    },
};
use anyhow::Result;

/// Mark the selected options contract as assigned
#[poise::command(slash_command)]
pub async fn assign(ctx: AppContext<'_>) -> Result<(), Error> {
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

    let last_index = position.contracts.len() - 1;
    position.contracts[last_index].open.status = "assigned".to_string();
    let q = position.contracts[last_index].open.quantity;
    let ticker = position.contracts[last_index].open.ticker.clone();

    position_list_replace(
        &mut db,
        "positions",
        indexed_position.index as usize,
        position,
    );

    ctx.say(format!("Assigned {} shares of {}", q * 100, ticker))
        .await?;

    db.set("edit_id", &-1)?;
    Ok(())
}
