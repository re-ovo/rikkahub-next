use sqlx::PgPool;
use uuid::Uuid;

use crate::models::User;

pub struct UserService;

impl UserService {
    pub async fn find_by_username(
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users WHERE username = $1 AND deleted_at IS NULL")
            .bind(username)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL")
            .bind(id)
            .fetch_optional(pool)
            .await
    }
}
