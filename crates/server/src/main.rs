//! RikkaHub 独立服务器
//!
//! 用于部署到服务器/Docker，提供 API 服务

mod handlers;
mod state;

use anyhow::Result;
use axum::Router;
use state::AppState;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// 创建 API 路由
fn create_router(state: AppState) -> Router {
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

#[tokio::main]
async fn main() -> Result<()> {
    // 加载 .env 文件
    dotenvy::dotenv().ok();

    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // 读取配置
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    // 创建应用状态
    let state = AppState::new();

    // 创建路由
    let app = create_router(state);

    // 启动服务器
    tracing::info!("RikkaHub Server 启动于 http://{}", addr);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
