use std::sync::Arc;

use database::db::DbPool;
use realtime::websockets::WsState;

pub mod auth;
pub mod blockchain;
pub mod cache;
pub mod config;
pub mod database;
pub mod dto;
pub mod errors;
pub mod github;
pub mod jobs;
pub mod middleware;
pub mod models;
pub mod realtime;
pub mod services;
pub mod utils;

pub struct AppState {
    pub pool: DbPool,
    pub ws_state: Arc<WsState>,
}
