//! API 路由处理器

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use rikkahub_core::{ChatRequest, ChatResponse, Conversation, Message, Model, Role};

use crate::AppState;

/// 聊天相关路由
pub fn chat_routes() -> Router<AppState> {
    Router::new().route("/send", post(send_message))
}

/// 模型相关路由
pub fn model_routes() -> Router<AppState> {
    Router::new().route("/", get(list_models))
}

/// 会话相关路由
pub fn conversation_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_conversations))
        .route("/", post(create_conversation))
}

/// 发送消息
async fn send_message(
    State(_state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Json<ChatResponse> {
    // TODO: 实际调用 AI 服务
    tracing::info!("收到聊天请求: {:?}", request);

    Json(ChatResponse {
        message: Message {
            role: Role::Assistant,
            content: "这是一个示例回复".to_string(),
        },
        usage: None,
    })
}

/// 获取模型列表
async fn list_models(State(_state): State<AppState>) -> Json<Vec<Model>> {
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

/// 获取会话列表
async fn list_conversations(State(_state): State<AppState>) -> Json<Vec<Conversation>> {
    // TODO: 从数据库获取
    Json(vec![])
}

/// 创建会话
async fn create_conversation(
    State(_state): State<AppState>,
    Json(title): Json<String>,
) -> Json<Conversation> {
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
