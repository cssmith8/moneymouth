use crate::types::types::{AppContext, Error};
use crate::utils::db::{
    get_options_db_path, get_selected_position, open_options_db, position_list_replace,
};
use crate::utils::log::log;
use anyhow::Result;
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Split Contract"] // Struct name by default
pub struct SplitModal {
    #[name = "Split Quantity"]
    #[placeholder = "1"]
    quantity: String,
}

//command that splits an existing position into 2 copies, each with less quantity
#[poise::command(slash_command)]
pub async fn split(ctx: AppContext<'_>) -> Result<(), Error> {
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
    let original_quantity = position.contracts[last_index].open.quantity;

    // Execute the modal to get split quantity
    let data = match SplitModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    let split_quantity = data.quantity.parse::<u16>()?;

    //make sure the provided quantity is between 0 and the original quantity, exclusive
    if split_quantity == 0 || split_quantity >= original_quantity {
        ctx.say("Split quantity must be greater than 0 and less than the original quantity")
            .await?;
        return Ok(());
    }

    // Adjust the quantity of all contracts in the selected position to subtract the given amount
    for contract in &mut position.contracts {
        contract.open.quantity = contract.open.quantity.saturating_sub(split_quantity);
    }

    // Create a duplicate of the original position
    let mut duplicate_position = position.clone();

    // Set the quantity of all contracts in the duplicate to the split amount
    for contract in &mut duplicate_position.contracts {
        contract.open.quantity = split_quantity;
    }

    //save the updated original position
    position_list_replace(
        &mut db,
        "positions",
        indexed_position.index as usize,
        position,
    );

    //save the new position
    db.ladd("positions", &duplicate_position)
        .ok_or_else(|| Error::from("Failed to add split position to database"))?;

    ctx.say(&format!(
        "Position split successfully. Original quantity: {}, Split quantity: {}",
        original_quantity - split_quantity,
        split_quantity
    ))
    .await?;

    Ok(())
}
