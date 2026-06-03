// src/bin/oauth_listener.rs
use std::net::SocketAddr;
use axum::{routing::get, Router, extract::Query};
use serde::Deserialize;
use tokio::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct CallbackParams {
    code: String,
    // optionally other params
}

async fn handle_callback(Query(params): Query<CallbackParams>) -> &'static str {
    // Write the code to a temporary file in the project root
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("oauth_code.txt");
    // ignore errors for simplicity
    let _ = fs::write(&path, &params.code).await;
    println!("Received OAuth code: {}", params.code);
    "OAuth code received. You can close this window."
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handle_callback));
    // Bind to 127.0.0.1:58263 as per plan
    let addr = SocketAddr::from(([127, 0, 0, 1], 58263));
    println!("OAuth listener running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
