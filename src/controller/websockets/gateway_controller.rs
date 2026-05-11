use crate::security::user_auth_guard::AuthenticatedUser;
use crate::services::gateway::gateway_service::GatewayService;
use rocket::State;
use rocket::futures::{SinkExt, StreamExt};
use rocket::serde::Deserialize;
use rocket::serde::json::serde_json;
use uuid::Uuid;
use crate::models::postgres::message::CreateMessage;

#[derive(Deserialize, Debug)]
struct IncomingMessage {
    #[serde(rename = "type")]
    msg_type: String,
    channel_id: Uuid,
    message: String,
    parent: Option<Uuid>,
    author_username: String,
    opti_id: String
}

#[rocket::get("/gateway")]
pub async fn gateway(
    ws: ws::WebSocket,
    gateway: &State<GatewayService>,
    user: AuthenticatedUser,
) -> ws::Channel<'_> {
        let gateway = gateway.inner();

    ws.channel(move |stream| {
        Box::pin(async move {
            let (mut sender, mut receiver) = stream.split();

            let tx = gateway.get_or_create(&user).await;
            let mut rx = tx.subscribe();

            loop {
                tokio::select! {
                    // incoming from user
                    msg = receiver.next() => {
                        match msg {
                            Some(Ok(ws::Message::Text(text))) => {
                                match serde_json::from_str::<IncomingMessage>(&text) {
                                    Ok(parsed) => {
                                        if !gateway.postgres.is_member(parsed.channel_id, user.clone()).await {
                                            let err = serde_json::json!({
                                                "type": "error",
                                                "message": "You are not a member of this channel",
                                            }).to_string();
                                            if sender.send(ws::Message::Text(err)).await.is_err() {
                                                break;
                                            }
                                            continue;
                                        }

                                        match parsed.msg_type.as_str() {
                                            "message" => {
                                                let saved = gateway.postgres.save_message(&CreateMessage {
                                                    channel_id: parsed.channel_id,
                                                    user_id: user.clone(),
                                                    content: parsed.message.clone(),
                                                    parent_id: parsed.parent,
                                                }).await;

                                                match saved {
                                                    Ok(msg) => {
                                                        let response = serde_json::json!({
                                                            "type": "message",
                                                            "message_id": msg.id,
                                                            "channel_id": msg.channel_id,
                                                            "user_id": msg.user_id,
                                                            "message": msg.content,
                                                            "parent": msg.parent_id,
                                                            "created_at": msg.created_at,
                                                            "author_username": parsed.author_username,
                                                            "opti_id": parsed.opti_id
                                                        }).to_string();
                                                        gateway.redis.publish(parsed.channel_id, response.clone()).await;
                                                        gateway.broadcast_message(parsed.channel_id, response).await;
                                                        gateway.postgres.update_last_read(parsed.channel_id, user.clone()).await;
                                                    }
                                                    Err(e) => {
                                                        println!("Failed to save message: {}", e);
                                                        let err = serde_json::json!({
                                                            "type": "error",
                                                            "message": "Failed to save message",
                                                        }).to_string();
                                                        if sender.send(ws::Message::Text(err)).await.is_err() {
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                println!("Unknown message type: {}", parsed.msg_type);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!("Invalid JSON: {}", e);
                                    }
                                }
                            }

                            Some(Err(e)) => {
                                println!("WebSocket error: {}", e);
                                break;
                            }

                            None => {
                                println!("Client disconnected.");
                                break;
                            }
                            _ => {}
                        }
                    }

                    // outgoing gateway
                    msg_from_gateway = rx.recv() => {
                        if let Ok(text) = msg_from_gateway {
                            if let Err(_) = sender.send(ws::Message::Text(text)).await {
                                break;
                            }
                        }
                    }
                }
            }

            gateway.cleanup().await;

            Ok(())
        })
    })
}
