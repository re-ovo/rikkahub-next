use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserGroup {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub created_at: DateTime<Utc>,
}

impl UserGroup {
    pub fn new(user_id: Uuid, group_id: Uuid) -> Self {
        Self {
            user_id,
            group_id,
            created_at: Utc::now(),
        }
    }
}
