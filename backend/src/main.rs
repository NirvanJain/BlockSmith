use axum::{
    routing::get,
    Router,
};

use dotenvy::dotenv;
use std::net::SocketAddr;

use blocksmith::{
    config::{
        app::AppConfig,
        database::DatabaseConfig,
    },
    database::connection_pool::create_pool,
    middleware::cors::cors_layer,
};

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_config =
        AppConfig::from_env();

    let database_config =
        DatabaseConfig::from_env();

    let _pool = create_pool(
        &database_config.database_url,
    )
    .await
    .expect(
        "Failed to connect database",
    );

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                "BlockSmith API Running"
            }),
        )
        .merge(routes::create_routes())
        .layer(cors_layer());

    let address = SocketAddr::from((
        [127, 0, 0, 1],
        app_config.port,
    ));

    println!(
        "Server running on {}",
        address
    );

let listener =
    tokio::net::TcpListener::bind(
        address,
    )
    .await
    .unwrap();

axum::serve(
    listener,
    app,
)
.await
.unwrap();
}