pub mod entities;
mod handlers;
pub mod middleware;
pub mod repositories;
mod state;

use anyhow::Result;
use axum::Router;
use middleware::JwtConfig;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // 创建 JWT 配置
    let jwt_config = JwtConfig::new(&jwt_secret);

    // 创建数据库连接池
    tracing::info!("正在连接数据库...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    // 运行数据库迁移
    tracing::info!("正在运行数据库迁移...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 创建应用状态
    let state = AppState::new(pool, jwt_config);

    // 创建路由
    let app = create_router(state);

    // 启动服务器
    tracing::info!("RikkaHub Server 启动于 http://{}", addr);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 创建 API 路由
fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .nest("/chat", handlers::chat_routes())
        .nest("/models", handlers::model_routes())
        .nest("/conversations", handlers::conversation_routes())
        .nest("/user", handlers::user_routes());

    Router::new()
        .nest("/api", api_routes)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}