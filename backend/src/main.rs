use axum::routing::get;
use axum::Router;

use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

use blocksmith::{
    config::app::AppConfig,
    database::connection_pool::create_pool,
    middleware::cors::cors_layer,
    realtime::websockets::WsState,
    AppState,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_config = AppConfig::from_env();

    let pool = create_pool()
        .await
        .expect("Failed to connect to MongoDB");

    println!("Connected to MongoDB");

    blocksmith::database::migrations::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let ws_state = Arc::new(WsState::default());

    let state = Arc::new(AppState { pool, ws_state });

    let app = Router::new()
        .route(
            "/",
            get(|| async { "BlockSmith API Running" }),
        )
        .merge(blocksmith::routes::create_routes())
        .with_state(state)
        .layer(cors_layer());

    let address = SocketAddr::from(([127, 0, 0, 1], app_config.port));

    println!("Server running on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
