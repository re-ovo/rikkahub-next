use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("请求失败: {0}")]
    RequestFailed(String),

    #[error("模型不存在: {0}")]
    ModelNotFound(String),

    #[error("会话不存在: {0}")]
    ConversationNotFound(String),

    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("内部错误: {0}")]
    Internal(String),
}
