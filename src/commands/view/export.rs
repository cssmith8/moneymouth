use crate::types::output::excelline::Excelline;
use crate::types::position::Position;
use crate::types::types::{AppContext, Error};
use crate::utils::db::{get_options_db_path, open_options_db};
use poise::serenity_prelude::CreateAttachment;

/// Export all options data to a CSV file
#[poise::command(slash_command)]
pub async fn export(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = get_options_db_path(userid.to_string());

    //immutable db
    let db = match open_options_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };

    let mut all_positions: Vec<Position> = Vec::new();
    // iterate over the items in list1
    for item_iter in db.liter("positions") {
        all_positions.push(item_iter.get_item::<Position>().unwrap());
    }

    // Collect all Excelline entries from all contracts in all positions
    let mut all_excellines: Vec<Excelline> = Vec::new();
    for position in &all_positions {
        for contract in &position.contracts {
            // Always add open_to_excelline
            all_excellines.push(contract.open_to_excelline());
            // If contract has a close, add close_to_excelline
            if let Some(close_line) = contract.close_to_excelline() {
                all_excellines.push(close_line);
            }
        }
    }

    // Sort by date (ascending)
    all_excellines.sort_by(|a, b| {
        (a.date.year, a.date.month, a.date.day).cmp(&(b.date.year, b.date.month, b.date.day))
    });

    // Format as CSV lines (no header)
    let mut output = String::new();
    for line in &all_excellines {
        output.push_str(&line.to_string());
        output.push('\n');
    }

    let reply = poise::CreateReply::default()
        .attachment(CreateAttachment::bytes(output.as_bytes(), "export.csv"));
    ctx.send(reply).await?;
    Ok(())
}
