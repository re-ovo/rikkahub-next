//! API 路由处理器

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use core::{ChatRequest, ChatResponse, Conversation, Message, Model, Role};

use crate::middleware::{AuthUser, OptionalAuthUser};
use crate::AppState;

/// 聊天相关路由 (需要认证)
pub fn chat_routes() -> Router<AppState> {
    Router::new().route("/send", post(send_message))
}

/// 模型相关路由 (公开)
pub fn model_routes() -> Router<AppState> {
    Router::new().route("/", get(list_models))
}

/// 会话相关路由 (需要认证)
pub fn conversation_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_conversations))
        .route("/", post(create_conversation))
}

/// 用户相关路由
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_current_user))
}

/// 发送消息 (需要认证)
async fn send_message(
    auth: AuthUser,
    State(_state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Json<ChatResponse> {
    // TODO: 实际调用 AI 服务
    tracing::info!("用户 {} 发送聊天请求: {:?}", auth.user_id, request);

    Json(ChatResponse {
        message: Message {
            role: Role::Assistant,
            content: "这是一个示例回复".to_string(),
        },
        usage: None,
    })
}

/// 获取模型列表 (公开，可选认证)
async fn list_models(
    auth: OptionalAuthUser,
    State(_state): State<AppState>,
) -> Json<Vec<Model>> {
    if let Some(user) = auth.0 {
        tracing::info!("用户 {} 获取模型列表", user.user_id);
    }

    // TODO: 从配置或数据库获取
    Json(vec![
        Model {
            id: "gpt-4".to_string(),
            name: "GPT-4".to_string(),
            provider: "OpenAI".to_string(),
        },
        Model {
            id: "claude-3".to_string(),
            name: "Claude 3".to_string(),
            provider: "Anthropic".to_string(),
        },
    ])
}

/// 获取会话列表 (需要认证)
async fn list_conversations(
    auth: AuthUser,
    State(_state): State<AppState>,
) -> Json<Vec<Conversation>> {
    tracing::info!("用户 {} 获取会话列表", auth.user_id);
    // TODO: 从数据库获取该用户的会话
    Json(vec![])
}

/// 创建会话 (需要认证)
async fn create_conversation(
    auth: AuthUser,
    State(_state): State<AppState>,
    Json(title): Json<String>,
) -> Json<Conversation> {
    tracing::info!("用户 {} 创建会话: {}", auth.user_id, title);
    // TODO: 保存到数据库
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    Json(Conversation {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        created_at: now,
        updated_at: now,
    })
}

/// 获取当前用户信息
async fn get_current_user(auth: AuthUser) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "user_id": auth.user_id,
        "username": auth.username,
    }))
}
