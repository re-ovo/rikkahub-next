//! JWT 认证中间件

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

/// JWT 配置
#[derive(Clone)]
pub struct JwtConfig {
    /// 编码密钥
    encoding_key: EncodingKey,
    /// 解码密钥
    decoding_key: DecodingKey,
    /// Access Token 过期时间(秒)
    pub access_token_expiry: i64,
    /// Refresh Token 过期时间(秒)
    pub refresh_token_expiry: i64,
    /// 签发者
    pub issuer: String,
}

impl JwtConfig {
    /// 从密钥字符串创建配置
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_token_expiry: 3600,      // 1 小时
            refresh_token_expiry: 604800,   // 7 天
            issuer: "rikkahub".to_string(),
        }
    }

    /// 设置 Access Token 过期时间
    pub fn with_access_expiry(mut self, seconds: i64) -> Self {
        self.access_token_expiry = seconds;
        self
    }

    /// 设置 Refresh Token 过期时间
    pub fn with_refresh_expiry(mut self, seconds: i64) -> Self {
        self.refresh_token_expiry = seconds;
        self
    }

    /// 设置签发者
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = issuer.into();
        self
    }

    /// 生成 Access Token
    pub fn generate_access_token(&self, user_id: Uuid, username: Option<String>) -> Result<String, AuthError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.access_token_expiry);

        let claims = Claims {
            sub: user_id.to_string(),
            username,
            iat: now.timestamp(),
            exp: exp.timestamp(),
            iss: self.issuer.clone(),
            token_type: TokenType::Access,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::TokenCreation(e.to_string()))
    }

    /// 生成 Refresh Token
    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String, AuthError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.refresh_token_expiry);

        let claims = Claims {
            sub: user_id.to_string(),
            username: None,
            iat: now.timestamp(),
            exp: exp.timestamp(),
            iss: self.issuer.clone(),
            token_type: TokenType::Refresh,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AuthError::TokenCreation(e.to_string()))
    }

    /// 生成 Token 对 (access + refresh)
    pub fn generate_token_pair(&self, user_id: Uuid, username: Option<String>) -> Result<TokenPair, AuthError> {
        Ok(TokenPair {
            access_token: self.generate_access_token(user_id, username)?,
            refresh_token: self.generate_refresh_token(user_id)?,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_expiry,
        })
    }

    /// 验证并解析 Token
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.issuer]);
        validation.validate_exp = true;
        validation.leeway = 0; // 无宽限期

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                jsonwebtoken::errors::ErrorKind::InvalidToken => AuthError::InvalidToken,
                _ => AuthError::InvalidToken,
            })
    }
}

/// Token 类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Access,
    Refresh,
}

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 ID
    pub sub: String,
    /// 用户名
    pub username: Option<String>,
    /// 签发时间
    pub iat: i64,
    /// 过期时间
    pub exp: i64,
    /// 签发者
    pub iss: String,
    /// Token 类型
    pub token_type: TokenType,
}

impl Claims {
    /// 获取用户 ID
    pub fn user_id(&self) -> Result<Uuid, AuthError> {
        Uuid::parse_str(&self.sub).map_err(|_| AuthError::InvalidToken)
    }

    /// 检查是否为 Access Token
    pub fn is_access_token(&self) -> bool {
        self.token_type == TokenType::Access
    }

    /// 检查是否为 Refresh Token
    pub fn is_refresh_token(&self) -> bool {
        self.token_type == TokenType::Refresh
    }
}

/// Token 对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

/// 认证错误
#[derive(Debug, Clone)]
pub enum AuthError {
    /// 缺少认证头
    MissingCredentials,
    /// Token 无效
    InvalidToken,
    /// Token 已过期
    TokenExpired,
    /// Token 类型错误
    WrongTokenType,
    /// Token 创建失败
    TokenCreation(String),
    /// 用户不存在
    UserNotFound,
    /// 用户已被禁用
    UserSuspended,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "缺少认证凭证"),
            Self::InvalidToken => write!(f, "无效的 Token"),
            Self::TokenExpired => write!(f, "Token 已过期"),
            Self::WrongTokenType => write!(f, "Token 类型错误"),
            Self::TokenCreation(e) => write!(f, "Token 创建失败: {}", e),
            Self::UserNotFound => write!(f, "用户不存在"),
            Self::UserSuspended => write!(f, "用户已被禁用"),
        }
    }
}

impl std::error::Error for AuthError {}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::MissingCredentials => (StatusCode::UNAUTHORIZED, "缺少认证凭证"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "无效的 Token"),
            Self::TokenExpired => (StatusCode::UNAUTHORIZED, "Token 已过期"),
            Self::WrongTokenType => (StatusCode::BAD_REQUEST, "Token 类型错误"),
            Self::TokenCreation(_) => (StatusCode::INTERNAL_SERVER_ERROR, "服务器错误"),
            Self::UserNotFound => (StatusCode::UNAUTHORIZED, "用户不存在"),
            Self::UserSuspended => (StatusCode::FORBIDDEN, "用户已被禁用"),
        };

        let body = Json(serde_json::json!({
            "error": message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

/// 已认证用户 (从请求中提取)
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub claims: Claims,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 提取 Authorization 头
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        // 获取 JWT 配置
        let app_state = AppState::from_ref(state);
        let jwt_config = app_state.jwt();

        // 验证 Token
        let claims = jwt_config.validate_token(bearer.token())?;

        // 确保是 Access Token
        if !claims.is_access_token() {
            return Err(AuthError::WrongTokenType);
        }

        let user_id = claims.user_id()?;

        Ok(AuthUser {
            user_id,
            username: claims.username.clone(),
            claims,
        })
    }
}

/// 可选认证用户 (不强制要求认证)
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(OptionalAuthUser(
            AuthUser::from_request_parts(parts, state).await.ok()
        ))
    }
}

// FromRef trait，用于从 AppState 提取子状态
use axum::extract::FromRef;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let config = JwtConfig::new("test-secret-key-for-testing");
        let user_id = Uuid::new_v4();
        let username = Some("testuser".to_string());

        // 生成 Access Token
        let token = config.generate_access_token(user_id, username.clone()).unwrap();
        assert!(!token.is_empty());

        // 验证 Token
        let claims = config.validate_token(&token).unwrap();
        assert_eq!(claims.user_id().unwrap(), user_id);
        assert_eq!(claims.username, username);
        assert!(claims.is_access_token());
    }

    #[test]
    fn test_generate_token_pair() {
        let config = JwtConfig::new("test-secret-key-for-testing");
        let user_id = Uuid::new_v4();

        let pair = config.generate_token_pair(user_id, Some("user".to_string())).unwrap();

        // 验证 Access Token
        let access_claims = config.validate_token(&pair.access_token).unwrap();
        assert!(access_claims.is_access_token());

        // 验证 Refresh Token
        let refresh_claims = config.validate_token(&pair.refresh_token).unwrap();
        assert!(refresh_claims.is_refresh_token());
    }

    #[test]
    fn test_expired_token() {
        let config = JwtConfig::new("test-secret")
            .with_access_expiry(-60); // 60秒前过期

        let user_id = Uuid::new_v4();
        let token = config.generate_access_token(user_id, None).unwrap();

        let result = config.validate_token(&token);
        assert!(matches!(result, Err(AuthError::TokenExpired)));
    }

    #[test]
    fn test_invalid_token() {
        let config = JwtConfig::new("test-secret");
        let result = config.validate_token("invalid.token.here");
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }
}
