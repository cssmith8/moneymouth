use crate::types::output::dblog::DBLog;
use crate::types::types::Error;
use crate::utils::db::create_or_open_db;
use chrono::Utc;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use serenity::Http;
use std::env;
use std::sync::{Arc, OnceLock};

static HTTP_CLIENT: OnceLock<Arc<Http>> = OnceLock::new();

pub fn set_http_client(http: Arc<Http>) {
    let _ = HTTP_CLIENT.set(http);
}

#[allow(dead_code)]
pub fn log(message: String) -> Result<(), Error> {
    let mut db = create_or_open_db(format!(
        "{}/logs.db",
        env::var("DB_PATH").unwrap_or_else(|_| "data/".into())
    ));
    if !db.lexists("logs") {
        db.lcreate("logs")?;
    }
    db.ladd(
        "logs",
        &DBLog {
            timestamp: Utc::now(),
            message: message.clone(),
        },
    )
    .ok_or_else(|| Error::from("Failed to add log to database"))?;
    if db.get::<bool>("realtime").unwrap_or(false) {
        send_realtime_log(&message);
    }
    println!("[Logged]: {}", message);
    Ok(())
}

fn send_realtime_log(message: &str) {
    if let Some(http) = HTTP_CLIENT.get() {
        let channel = ChannelId::new(1160065321013620857);
        let message = message.to_string();
        let http = http.clone();
        tokio::spawn(async move {
            let _ = channel.say(&http, &message).await;
        });
    }
}

pub fn load_all_logs() -> Result<Vec<DBLog>, Error> {
    let db = create_or_open_db("data/logs.db".to_string());

    let mut all_logs: Vec<DBLog> = Vec::new();
    for item_iter in db.liter("logs") {
        let db_log = item_iter.get_item::<DBLog>().unwrap();
        all_logs.push(db_log);
    }
    Ok(all_logs)
}
