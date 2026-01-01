//! RikkaHub API 层
//!
//! 提供 axum 路由，可被桌面端内嵌或独立部署

mod handlers;
mod state;

use axum::Router;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

pub use state::AppState;

/// 创建 API 路由
pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/chat", handlers::chat_routes())
        .nest("/models", handlers::model_routes())
        .nest("/conversations", handlers::conversation_routes());

    Router::new()
        .nest("/api", api_routes)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
