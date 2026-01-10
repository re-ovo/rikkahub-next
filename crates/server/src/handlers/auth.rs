use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    middleware::{AppState, AuthUser},
    models::{User, UserStatus},
    services::UserService,
    utils::{jwt::encode_token, verify_password},
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user: Option<User> = UserService::find_by_username(&state.pool, &payload.username)
        .await
        .map_err(|e| {
            tracing::error!("数据库查询失败: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "内部服务器错误" })),
            )
        })?;

    let user = match user {
        Some(u) => u,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "用户名或密码错误" })),
            ));
        }
    };

    if user.status != UserStatus::Active as i16 {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "用户已被禁用" })),
        ));
    }

    let password_hash = match &user.password_hash {
        Some(hash) => hash,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "用户名或密码错误" })),
            ));
        }
    };

    let is_valid = verify_password(&payload.password, password_hash).map_err(|e| {
        tracing::error!("密码验证失败: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "内部服务器错误" })),
        )
    })?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "用户名或密码错误" })),
        ));
    }

    let token = encode_token(
        user.id,
        &state.config.jwt.secret,
        state.config.jwt.expires_in,
    )
    .map_err(|e| {
        tracing::error!("JWT 生成失败: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "内部服务器错误" })),
        )
    })?;

    Ok(Json(LoginResponse { token, user }))
}

pub async fn me(AuthUser { user, .. }: AuthUser) -> Json<User> {
    Json(user)
}
