//! RikkaHub 独立服务器
//!
//! 用于部署到服务器，提供 API 服务

use anyhow::Result;
use rikkahub_api::{create_router, AppState};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
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
