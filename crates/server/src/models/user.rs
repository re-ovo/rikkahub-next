use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "smallint")]
#[repr(i16)]
pub enum UserStatus {
    Disabled = 0,
    Active = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub nickname: String,
    pub avatar: String,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(username: String, password_hash: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username: username.clone(),
            email: None,
            password_hash,
            nickname: username,
            avatar: String::new(),
            status: UserStatus::Active as i16,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}
