use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroupPermission {
    pub id: Uuid,
    pub group_id: Uuid,
    pub permission: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl GroupPermission {
    pub fn new(group_id: Uuid, permission: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            group_id,
            permission,
            created_at: Utc::now(),
            deleted_at: None,
        }
    }
}
