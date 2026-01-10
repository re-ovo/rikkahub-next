use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::config::Config;

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "rikkahub-server"
    }))
}

pub fn create_router(pool: PgPool, config: Config) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state((pool, config))
}
