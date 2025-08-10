use crate::types::indexedposition::IndexedPosition;
use crate::types::position::Position;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::env;

pub fn create_or_open_db(path: String) -> PickleDb {
    let opendb = match PickleDb::load(
        path.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
        Err(_e) => {
            println!("Creating new db at: {}", path);
            PickleDb::new(
                path.clone(),
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )
        }
    };
    opendb
}

pub fn open_options_db(path: String) -> Option<PickleDb> {
    let mut new_flag = false;
    let mut opendb = match PickleDb::load(
        path.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
        Err(_e) => {
            println!("Creating new db at: {}", path);
            new_flag = true;
            PickleDb::new(
                path.clone(),
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )
        }
    };
    if new_flag {
        opendb.set("commission", &0.65).unwrap();
        opendb.set("edit_id", &-1).unwrap();
        opendb.lcreate("positions").unwrap();
    }
    Some(opendb)
}

pub fn position_list_replace(db: &mut PickleDb, name: &str, index: usize, position: Position) {
    //empty vector
    let mut vec: Vec<Position> = Vec::new();
    // iterate over the items in list1
    for item_iter in db.liter(name) {
        vec.push(item_iter.get_item::<Position>().unwrap());
    }
    //replace element at index
    vec[index] = position;

    db.lrem_list(name).unwrap();
    // create a new list
    db.lcreate(name).unwrap();
    db.lextend(name, &vec).unwrap();
}

pub fn get_options_db_path(userid: String) -> String {
    let path = env::var("DB_PATH").expect("0");
    format!("{}/options{}.db", path, userid)
}

pub fn get_selected_position(userid: String) -> Result<IndexedPosition, String> {
    let db_location = get_options_db_path(userid.to_string());

    let db = match open_options_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err("Could not load db".to_string());
        }
    };
    let edit_id: i32 = match db.get("edit_id") {
        Some(id) => id,
        None => {
            return Err("Failed to retrieve edit_id".to_string());
        }
    };
    if edit_id == -1 {
        return Err("No open position selected".to_string());
    }
    if edit_id >= db.llen("positions") as i32 {
        return Err("Invalid selection".to_string());
    }
    let position: Position = match db.lget("positions", edit_id as usize) {
        Some(pos) => pos,
        None => {
            return Err("Failed to retrieve position".to_string());
        }
    };
    Ok(IndexedPosition {
        position,
        index: edit_id,
    })
}
