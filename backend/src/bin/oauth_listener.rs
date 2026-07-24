use std::net::SocketAddr;
use axum::{routing::get, Router, extract::Query};
use serde::Deserialize;
use tokio::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
}

async fn handle_callback(Query(params): Query<CallbackParams>) -> &'static str {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("oauth_code.txt");
    let _ = fs::write(&path, &params.code).await;
    println!("Received OAuth code: {}", params.code);
    "OAuth code received. You can close this window."
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle_callback));
    let addr = SocketAddr::from(([127, 0, 0, 1], 58263));
    println!("OAuth listener running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
