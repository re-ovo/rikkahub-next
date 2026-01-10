use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum SettingType {
    Bool,
    String,
    Int,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Setting {
    pub key: String,
    pub value: JsonValue,
    #[sqlx(rename = "type")]
    pub setting_type: String,
    pub description: String,
    pub updated_at: DateTime<Utc>,
}

impl Setting {
    pub fn new(
        key: String,
        value: JsonValue,
        setting_type: SettingType,
        description: String,
    ) -> Self {
        Self {
            key,
            value,
            setting_type: match setting_type {
                SettingType::Bool => "bool".to_string(),
                SettingType::String => "string".to_string(),
                SettingType::Int => "int".to_string(),
                SettingType::Json => "json".to_string(),
            },
            description,
            updated_at: Utc::now(),
        }
    }
}
