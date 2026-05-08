use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::fs;
use std::path::Path;

pub async fn init_db() -> SqlitePool {
   
    // creating db directory on startup 
    let db = "db";
    if !Path::new(db).exists() {
        fs::create_dir_all(db).expect("failed to create db");
    }

    // connection pooling with mode=rwc (read, write, create)
    let path = "sqlite://db/shell_db.sqlite?mode=rwc";
    let pool = SqlitePoolOptions::new()
                                .max_connections(10)
                                .connect(path)
                                .await
                                .expect("failed to connect to sqlite");

    // Creating schema and pragmas
    let setup = [
        "CREATE TABLE IF NOT EXISTS victims (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            uuid TEXT UNIQUE NOT NULL,
            token TEXT,
            verify INTEGER DEFAULT 0,
            last_seen INTEGER NOT NULL        
            );",
        "CREATE INDEX IF NOT EXISTS idx_uuid ON victims(uuid);",
        "CREATE INDEX IF NOT EXISTS idx_token ON victims(token);",
        "PRAGMA journal_mode=WAL;",
        "PRAGMA synchronous=NORMAL;",
    ];

    for query in setup {
        sqlx::query(query).execute(&pool).await.expect("error on database init");
    }

    pool
}
