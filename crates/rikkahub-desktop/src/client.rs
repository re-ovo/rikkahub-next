//! HTTP 客户端，用于与后端 API 通信

use anyhow::Result;
use rikkahub_core::{ChatRequest, ChatResponse, Conversation, Model};

/// API 客户端
pub struct ApiClient {
    base_url: String,
    http: reqwest::Client,
}

impl ApiClient {
    /// 创建新的 API 客户端
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http: reqwest::Client::new(),
        }
    }

    /// 本地模式（连接本地内嵌服务）
    pub fn local() -> Self {
        Self::new("http://localhost:3000")
    }

    /// 发送聊天消息
    pub async fn send_message(&self, request: ChatRequest) -> Result<ChatResponse> {
        let resp = self
            .http
            .post(format!("{}/api/chat/send", self.base_url))
            .json(&request)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// 获取模型列表
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        let resp = self
            .http
            .get(format!("{}/api/models", self.base_url))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// 获取会话列表
    pub async fn list_conversations(&self) -> Result<Vec<Conversation>> {
        let resp = self
            .http
            .get(format!("{}/api/conversations", self.base_url))
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }

    /// 创建新会话
    pub async fn create_conversation(&self, title: String) -> Result<Conversation> {
        let resp = self
            .http
            .post(format!("{}/api/conversations", self.base_url))
            .json(&title)
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }
}
