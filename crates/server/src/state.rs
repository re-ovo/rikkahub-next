//! 应用状态

use sqlx::PgPool;
use std::sync::Arc;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    /// 数据库连接池
    db: PgPool,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new(db: PgPool) -> Self {
        Self {
            inner: Arc::new(AppStateInner { db }),
        }
    }

    /// 获取数据库连接池
    pub fn db(&self) -> &PgPool {
        &self.inner.db
    }
}
