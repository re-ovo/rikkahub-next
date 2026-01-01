use std::sync::Arc;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    // TODO: 添加数据库连接、Redis、配置等
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AppStateInner {}),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
