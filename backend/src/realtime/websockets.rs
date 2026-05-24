use axum::{
    extract::ws::{
        Message,
        WebSocket,
        WebSocketUpgrade,
    },
    response::IntoResponse,
};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(
    mut socket: WebSocket,
) {
    println!("WebSocket connected");

    while let Some(message) =
        socket.recv().await
    {
        match message {
            Ok(Message::Text(text)) => {
                println!(
                    "Received message: {}",
                    text
                );

                let response = format!(
                    "BlockSmith Event: {}",
                    text
                );

                if socket
                    .send(Message::Text(
                        response.into(),
                    ))
                    .await
                    .is_err()
                {
                    break;
                }
            }

            Ok(Message::Close(_)) => {
                println!(
                    "WebSocket disconnected"
                );

                break;
            }

            _ => {}
        }
    }
}