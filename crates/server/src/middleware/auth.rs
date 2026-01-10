use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    models::{User, UserStatus},
    services::UserService,
    utils::jwt::decode_token,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
}

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub user: User,
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    ExpiredToken,
    UserNotFound,
    UserDisabled,
    InternalError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::MissingToken => (StatusCode::UNAUTHORIZED, "缺少认证令牌"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "无效的认证令牌"),
            Self::ExpiredToken => (StatusCode::UNAUTHORIZED, "认证令牌已过期"),
            Self::UserNotFound => (StatusCode::UNAUTHORIZED, "用户不存在"),
            Self::UserDisabled => (StatusCode::FORBIDDEN, "用户已被禁用"),
            Self::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "内部服务器错误"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;

        let claims = decode_token(token, &state.config.jwt.secret)
            .map_err(|e| {
                tracing::debug!("JWT 解码失败: {}", e);
                if e.to_string().contains("expired") {
                    AuthError::ExpiredToken
                } else {
                    AuthError::InvalidToken
                }
            })?;

        let user: User = UserService::find_by_id(&state.pool, claims.sub)
            .await
            .map_err(|e| {
                tracing::error!("数据库查询失败: {}", e);
                AuthError::InternalError
            })?
            .ok_or(AuthError::UserNotFound)?;

        if user.status != UserStatus::Active as i16 {
            return Err(AuthError::UserDisabled);
        }

        Ok(AuthUser {
            user_id: claims.sub,
            user,
        })
    }
}
