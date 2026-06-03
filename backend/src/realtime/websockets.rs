use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State, Query},
    response::IntoResponse,
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::broadcast;
use futures_util::{StreamExt, SinkExt};

pub struct WsState {
    pub feed_tx: broadcast::Sender<String>,
}

impl Default for WsState {
    fn default() -> Self {
        let (feed_tx, _) = broadcast::channel(100);
        Self { feed_tx }
    }
}

#[derive(Debug, Deserialize)]
pub struct WsParams {
    pub token: String,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(pool): State<Pool<Postgres>>,
    State(ws_state): State<Arc<WsState>>,
) -> impl IntoResponse {
    match crate::auth::jwt::verify_jwt(&params.token).await {
        Ok(claims) => {
            ws.on_upgrade(move |socket| handle_socket(socket, claims.sub, pool, ws_state))
        }
        Err(e) => {
            (StatusCode::UNAUTHORIZED, format!("Unauthorized: {}", e)).into_response()
        }
    }
}

async fn handle_socket(
    socket: WebSocket,
    clerk_user_id: String,
    pool: Pool<Postgres>,
    ws_state: Arc<WsState>,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut feed_rx = ws_state.feed_tx.subscribe();
    
    let clerk_id_clone = clerk_user_id.clone();
    let mut send_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                Ok(msg_text) = feed_rx.recv() => {
                    if sender.send(Message::Text(msg_text.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    let pool_clone = pool.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    if text == "ping" {
                        // ignore or respond
                    } else if let Ok(chat_payload) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(conv_id_str) = chat_payload.get("conversation_id").and_then(|v| v.as_str()) {
                            if let Some(content) = chat_payload.get("content").and_then(|v| v.as_str()) {
                                if let Ok(conv_uuid) = uuid::Uuid::parse_str(conv_id_str) {
                                    if let Ok(Some(sender_uuid)) = sqlx::query_scalar::<_, uuid::Uuid>(
                                        "SELECT id FROM users WHERE clerk_user_id = $1",
                                    )
                                    .bind(&clerk_id_clone)
                                    .fetch_optional(&pool_clone)
                                    .await {
                                        let _ = sqlx::query(
                                            "INSERT INTO messages (conversation_id, sender_id, content) VALUES ($1, $2, $3)"
                                        )
                                            .bind(conv_uuid)
                                            .bind(sender_uuid)
                                            .bind(content)
                                            .execute(&pool_clone)
                                            .await;
                                    }
                                }
                            }
                        }
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
    
    println!("WebSocket connection closed for user: {}", clerk_user_id);
}