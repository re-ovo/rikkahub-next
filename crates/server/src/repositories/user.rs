//! 用户数据访问

use crate::entities::*;
use sqlx::PgPool;
use uuid::Uuid;

/// 用户仓库
pub struct UserRepository;

impl UserRepository {
    /// 根据 ID 查询用户
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// 根据用户名查询用户
    pub async fn find_by_username(pool: &PgPool, username: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await
    }

    /// 根据邮箱查询用户
    pub async fn find_by_email(pool: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await
    }

    /// 创建用户
    pub async fn create(pool: &PgPool, req: CreateUser) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, display_name, avatar_url)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&req.username)
        .bind(&req.email)
        .bind(&req.display_name)
        .bind(&req.avatar_url)
        .fetch_one(pool)
        .await
    }

    /// 更新用户
    pub async fn update(pool: &PgPool, id: Uuid, req: UpdateUser) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET
                username = COALESCE($2, username),
                email = COALESCE($3, email),
                display_name = COALESCE($4, display_name),
                avatar_url = COALESCE($5, avatar_url),
                status = COALESCE($6, status),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&req.username)
        .bind(&req.email)
        .bind(&req.display_name)
        .bind(&req.avatar_url)
        .bind(req.status.map(|s| s.to_string()))
        .fetch_optional(pool)
        .await
    }

    /// 删除用户（软删除）
    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query(
            "UPDATE users SET status = 'deleted', updated_at = NOW() WHERE id = $1 AND status != 'deleted'",
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 分页查询用户
    pub async fn list(pool: &PgPool, offset: i64, limit: i64) -> sqlx::Result<Vec<User>> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE status != 'deleted' ORDER BY created_at DESC OFFSET $1 LIMIT $2",
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
}

/// 用户凭证仓库
pub struct CredentialRepository;

impl CredentialRepository {
    /// 根据用户 ID 查询凭证
    pub async fn find_by_user_id(
        pool: &PgPool,
        user_id: Uuid,
    ) -> sqlx::Result<Option<UserCredential>> {
        sqlx::query_as::<_, UserCredential>("SELECT * FROM user_credentials WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    /// 创建凭证
    pub async fn create(pool: &PgPool, req: CreateCredential) -> sqlx::Result<UserCredential> {
        sqlx::query_as::<_, UserCredential>(
            r#"
            INSERT INTO user_credentials (user_id, password_hash)
            VALUES ($1, $2)
            RETURNING *
            "#,
        )
        .bind(req.user_id)
        .bind(&req.password_hash)
        .fetch_one(pool)
        .await
    }

    /// 更新密码
    pub async fn update_password(
        pool: &PgPool,
        user_id: Uuid,
        password_hash: &str,
    ) -> sqlx::Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE user_credentials SET
                password_hash = $2,
                password_changed_at = NOW(),
                failed_attempts = 0,
                locked_until = NULL
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .bind(password_hash)
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 增加登录失败次数
    pub async fn increment_failed_attempts(pool: &PgPool, user_id: Uuid) -> sqlx::Result<i32> {
        let record = sqlx::query_as::<_, (i32,)>(
            r#"
            UPDATE user_credentials SET
                failed_attempts = failed_attempts + 1
            WHERE user_id = $1
            RETURNING failed_attempts
            "#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(record.0)
    }

    /// 重置登录失败次数
    pub async fn reset_failed_attempts(pool: &PgPool, user_id: Uuid) -> sqlx::Result<()> {
        sqlx::query(
            "UPDATE user_credentials SET failed_attempts = 0, locked_until = NULL WHERE user_id = $1",
        )
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// 锁定账号
    pub async fn lock_until(
        pool: &PgPool,
        user_id: Uuid,
        until: chrono::DateTime<chrono::Utc>,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE user_credentials SET locked_until = $2 WHERE user_id = $1")
            .bind(user_id)
            .bind(until)
            .execute(pool)
            .await?;
        Ok(())
    }
}

/// OAuth 账号仓库
pub struct OAuthRepository;

impl OAuthRepository {
    /// 根据 provider 和 provider_user_id 查询
    pub async fn find_by_provider(
        pool: &PgPool,
        provider: &str,
        provider_user_id: &str,
    ) -> sqlx::Result<Option<OAuthAccount>> {
        sqlx::query_as::<_, OAuthAccount>(
            "SELECT * FROM oauth_accounts WHERE provider = $1 AND provider_user_id = $2",
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(pool)
        .await
    }

    /// 查询用户的所有 OAuth 账号
    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<OAuthAccount>> {
        sqlx::query_as::<_, OAuthAccount>("SELECT * FROM oauth_accounts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    /// 创建 OAuth 账号
    pub async fn create(pool: &PgPool, req: CreateOAuthAccount) -> sqlx::Result<OAuthAccount> {
        sqlx::query_as::<_, OAuthAccount>(
            r#"
            INSERT INTO oauth_accounts (user_id, provider, provider_user_id, access_token, refresh_token, token_expires_at, raw_data)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(req.user_id)
        .bind(&req.provider)
        .bind(&req.provider_user_id)
        .bind(&req.access_token)
        .bind(&req.refresh_token)
        .bind(req.token_expires_at)
        .bind(&req.raw_data)
        .fetch_one(pool)
        .await
    }

    /// 更新 OAuth tokens
    pub async fn update_tokens(
        pool: &PgPool,
        id: Uuid,
        access_token: Option<&str>,
        refresh_token: Option<&str>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> sqlx::Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE oauth_accounts SET
                access_token = COALESCE($2, access_token),
                refresh_token = COALESCE($3, refresh_token),
                token_expires_at = COALESCE($4, token_expires_at),
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id)
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 删除 OAuth 账号
    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query("DELETE FROM oauth_accounts WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

/// Passkey 仓库
pub struct PasskeyRepository;

impl PasskeyRepository {
    /// 根据 credential_id 查询
    pub async fn find_by_credential_id(
        pool: &PgPool,
        credential_id: &[u8],
    ) -> sqlx::Result<Option<Passkey>> {
        sqlx::query_as::<_, Passkey>("SELECT * FROM passkeys WHERE credential_id = $1")
            .bind(credential_id)
            .fetch_optional(pool)
            .await
    }

    /// 查询用户的所有 Passkey
    pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<Passkey>> {
        sqlx::query_as::<_, Passkey>("SELECT * FROM passkeys WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    /// 创建 Passkey
    pub async fn create(pool: &PgPool, req: CreatePasskey) -> sqlx::Result<Passkey> {
        sqlx::query_as::<_, Passkey>(
            r#"
            INSERT INTO passkeys (user_id, credential_id, public_key, device_name, transports, aaguid)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(req.user_id)
        .bind(&req.credential_id)
        .bind(&req.public_key)
        .bind(&req.device_name)
        .bind(&req.transports)
        .bind(&req.aaguid)
        .fetch_one(pool)
        .await
    }

    /// 更新 sign_count（防重放攻击）
    pub async fn update_sign_count(
        pool: &PgPool,
        credential_id: &[u8],
        sign_count: i64,
    ) -> sqlx::Result<bool> {
        let result = sqlx::query(
            "UPDATE passkeys SET sign_count = $2, last_used_at = NOW() WHERE credential_id = $1",
        )
        .bind(credential_id)
        .bind(sign_count)
        .execute(pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 删除 Passkey
    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query("DELETE FROM passkeys WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

/// 认证日志仓库
pub struct AuthLogRepository;

impl AuthLogRepository {
    /// 创建日志
    pub async fn create(pool: &PgPool, req: CreateAuthLog) -> sqlx::Result<AuthLog> {
        sqlx::query_as::<_, AuthLog>(
            r#"
            INSERT INTO auth_logs (user_id, action, auth_method, ip_address, user_agent)
            VALUES ($1, $2, $3, $4::INET, $5)
            RETURNING id, user_id, action, auth_method, ip_address::TEXT, user_agent, created_at
            "#,
        )
        .bind(req.user_id)
        .bind(req.action.to_string())
        .bind(req.auth_method.map(|m| m.to_string()))
        .bind(&req.ip_address)
        .bind(&req.user_agent)
        .fetch_one(pool)
        .await
    }

    /// 查询用户的认证日志
    pub async fn find_by_user_id(
        pool: &PgPool,
        user_id: Uuid,
        limit: i64,
    ) -> sqlx::Result<Vec<AuthLog>> {
        sqlx::query_as::<_, AuthLog>(
            "SELECT id, user_id, action, auth_method, ip_address::TEXT, user_agent, created_at FROM auth_logs WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(pool)
        .await
    }
}
