//! 身份组相关实体

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 身份组
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Group {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
}

/// 创建身份组请求
#[derive(Debug, Clone, Deserialize)]
pub struct CreateGroup {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

/// 更新身份组请求
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGroup {
    pub name: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
}

/// 组成员
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroupMember {
    pub group_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

/// 添加组成员请求
#[derive(Debug, Clone)]
pub struct AddGroupMember {
    pub group_id: Uuid,
    pub user_id: Uuid,
}

/// 身份组权限
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GroupPermission {
    pub group_id: Uuid,
    pub permission: String,
}

/// 带详情的组（用于查询）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupWithDetails {
    pub group: Group,
    pub member_count: i64,
    pub permissions: Vec<String>,
}

/// 组树节点（用于层级展示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTreeNode {
    pub group: Group,
    pub children: Vec<GroupTreeNode>,
}
