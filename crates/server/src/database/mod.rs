use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::DatabaseConfig;
use crate::utils::hash_password;

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub async fn seed(pool: &PgPool) -> Result<()> {
    // 检查是否已经有数据
    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if user_count.0 > 0 {
        tracing::info!("数据库已有数据，跳过 seed");
        return Ok(());
    }

    tracing::info!("开始初始化种子数据...");

    // 创建用户组: users 和 admin
    let users_group_id: (uuid::Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO groups (name, description, is_default)
        VALUES ('users', '普通用户组', true)
        RETURNING id
        "#,
    )
    .fetch_one(pool)
    .await?;

    let admin_group_id: (uuid::Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO groups (name, description, is_default)
        VALUES ('admin', '管理员组', false)
        RETURNING id
        "#,
    )
    .fetch_one(pool)
    .await?;

    tracing::info!("用户组创建完成: users, admin");

    // 创建 admin 用户
    let password_hash = hash_password("admin")?;
    let admin_user_id: (uuid::Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO users (username, nickname, password_hash, status)
        VALUES ('admin', 'Administrator', $1, 1)
        RETURNING id
        "#,
    )
    .bind(&password_hash)
    .fetch_one(pool)
    .await?;

    tracing::info!("admin 用户创建完成");

    // 将 admin 用户加入 users 和 admin 组
    sqlx::query(
        r#"
        INSERT INTO user_groups (user_id, group_id)
        VALUES ($1, $2), ($1, $3)
        "#,
    )
    .bind(admin_user_id.0)
    .bind(users_group_id.0)
    .bind(admin_group_id.0)
    .execute(pool)
    .await?;

    tracing::info!("admin 用户已加入 users 和 admin 组");

    // 为 admin 组添加权限
    sqlx::query(
        r#"
        INSERT INTO group_permissions (group_id, permission)
        VALUES ($1, '*')
        "#,
    )
    .bind(admin_group_id.0)
    .execute(pool)
    .await?;

    tracing::info!("admin 组权限设置完成");
    tracing::info!("种子数据初始化完成！默认账户: admin/admin");

    Ok(())
}
