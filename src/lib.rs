pub mod sql;
pub mod routes;
pub mod payload;
pub mod bot;
pub mod index;

use dashmap::DashMap;
use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};
use teloxide::{adaptors::Throttle, prelude::*};
use sqlx::SqlitePool;

pub struct AppState {
    pub db: SqlitePool,     
    pub redirect: Arc<Mutex<String>>,
    pub command: Arc<DashMap<String, (String, u64)>>, 
    pub msg: Arc<DashMap<i32, u64>>,
    pub admin_id: i64,
    pub bot: Throttle<Bot>,
}

pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    .replace('"', "&quot;")
    .replace('\'', "&#039;")
}

pub fn get_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
