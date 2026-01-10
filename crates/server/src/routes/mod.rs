use axum::{Json, Router, routing::{get, post}};
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::config::Config;
use crate::handlers::{login, me};
use crate::middleware::AppState;

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "rikkahub-server"
    }))
}

pub fn create_router(pool: PgPool, config: Config) -> Router {
    let state = AppState { pool, config };

    Router::new()
        .route("/health", get(health_check))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
        .with_state(state)
}
