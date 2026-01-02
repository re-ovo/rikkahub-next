//! 应用状态

use sqlx::PgPool;
use std::sync::Arc;

use crate::middleware::JwtConfig;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    inner: Arc<AppStateInner>,
}

struct AppStateInner {
    /// 数据库连接池
    db: PgPool,
    /// JWT 配置
    jwt: JwtConfig,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new(db: PgPool, jwt: JwtConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { db, jwt }),
        }
    }

    /// 获取数据库连接池
    pub fn db(&self) -> &PgPool {
        &self.inner.db
    }

    /// 获取 JWT 配置
    pub fn jwt(&self) -> &JwtConfig {
        &self.inner.jwt
    }
}
