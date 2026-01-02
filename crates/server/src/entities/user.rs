//! 用户相关实体

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 用户状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Suspended,
    Deleted,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Suspended => write!(f, "suspended"),
            Self::Deleted => write!(f, "deleted"),
        }
    }
}

impl std::str::FromStr for UserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            "deleted" => Ok(Self::Deleted),
            _ => Err(format!("unknown user status: {}", s)),
        }
    }
}

/// 用户
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建用户请求
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// 更新用户请求
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: Option<UserStatus>,
}

/// 用户密码凭证
#[derive(Debug, Clone, FromRow)]
pub struct UserCredential {
    pub user_id: Uuid,
    pub password_hash: String,
    pub password_changed_at: DateTime<Utc>,
    pub failed_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
}

/// 创建密码凭证请求
#[derive(Debug, Clone)]
pub struct CreateCredential {
    pub user_id: Uuid,
    pub password_hash: String,
}

/// OAuth 账号
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub raw_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建 OAuth 账号请求
#[derive(Debug, Clone)]
pub struct CreateOAuthAccount {
    pub user_id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub raw_data: Option<serde_json::Value>,
}

/// Passkey (WebAuthn)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Passkey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub sign_count: i64,
    pub device_name: Option<String>,
    pub transports: Option<Vec<String>>,
    pub aaguid: Option<Vec<u8>>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// 创建 Passkey 请求
#[derive(Debug, Clone)]
pub struct CreatePasskey {
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
    pub device_name: Option<String>,
    pub transports: Option<Vec<String>>,
    pub aaguid: Option<Vec<u8>>,
}

/// 认证日志动作类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthAction {
    Login,
    Logout,
    FailedLogin,
    PasswordChange,
    PasskeyRegister,
    OAuthLink,
}

impl std::fmt::Display for AuthAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Login => write!(f, "login"),
            Self::Logout => write!(f, "logout"),
            Self::FailedLogin => write!(f, "failed_login"),
            Self::PasswordChange => write!(f, "password_change"),
            Self::PasskeyRegister => write!(f, "passkey_register"),
            Self::OAuthLink => write!(f, "oauth_link"),
        }
    }
}

/// 认证方法
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    Password,
    OAuth,
    Passkey,
}

impl std::fmt::Display for AuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Password => write!(f, "password"),
            Self::OAuth => write!(f, "oauth"),
            Self::Passkey => write!(f, "passkey"),
        }
    }
}

/// 认证日志
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuthLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub auth_method: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 创建认证日志请求
#[derive(Debug, Clone)]
pub struct CreateAuthLog {
    pub user_id: Option<Uuid>,
    pub action: AuthAction,
    pub auth_method: Option<AuthMethod>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
