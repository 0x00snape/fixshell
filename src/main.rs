use fixshell::{sql, AppState, get_now, routes::router};
use std::{env, net::SocketAddr, sync::{Arc, Mutex}, time::Duration};
use teloxide::{prelude::*, types::{ChatId, MessageId}, adaptors::throttle::Limits};
use tokio::{net::TcpListener, time::interval}; 
use dashmap::DashMap;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    // Database sqlite pool and shared state initialization 
    let db = sql::init_db().await;    
    let bot = Bot::new(env::var("BOT_TOKEN").expect("BOT_TOKEN missing")).throttle(Limits::default());

    let state = Arc::new(AppState {
        db: db, 
        redirect: Arc::new(Mutex::new("https://google.com".into())),
        command: Arc::new(DashMap::new()),
        msg: Arc::new(DashMap::new()),
        admin_id: env::var("ADMIN_ID").expect("ADMIN_ID missing").parse().unwrap(),
        bot,
    });

    // cleanup the messages and commands of bot after 10 min
    let cleanup = state.clone();
    tokio::spawn(async move {
        
        let mut interval = interval(Duration::from_secs(300)); 
        
        loop {
            interval.tick().await;

            cleanup.command.retain(|_, (_, expiry)| *expiry > get_now());

            cleanup.msg.retain(|id, expiry| {
                let expired = *expiry <= get_now();
                if expired {
                    let bot = cleanup.bot.clone();
                    let cid = ChatId(cleanup.admin_id);
                    let mid = MessageId(*id as i32);
                    tokio::spawn(async move {
                        let _ = bot.delete_message(cid, mid).await;
                    });
                }
                !expired 
            });
        }
    });
    
    // Building the axum router from routes.rs
    let app = router(state.clone()).await;
    let listener = TcpListener::bind("0.0.0.0:6969").await.unwrap();
    
    println!("{}", format!(
    r#"
    ╔══════════════════════════════════════╗
    ║               FIXSHELL               ║
    ║           v(1.0) -> ar.p             ║
    ╚══════════════════════════════════════╝
    "#));
    println!("Web Server and Reverse HTTP/S Handler running on http://0.0.0.0:6969");
    
    tokio::select! {
        _ = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()) => {},
        _ = fixshell::bot::run_bot(state.bot.clone(), state) => {}
    }
}
