//! 用户权限数据访问

use crate::entities::*;
use crate::repositories::group::{GroupMemberRepository, GroupPermissionRepository};
use sqlx::PgPool;
use uuid::Uuid;

/// 用户权限仓库
pub struct UserPermissionRepository;

impl UserPermissionRepository {
    /// 获取用户的所有权限（包括组继承）
    pub async fn get_user_permissions(
        pool: &PgPool,
        user_id: Uuid,
    ) -> sqlx::Result<UserPermissions> {
        // 获取用户所在的组及其祖先组
        let groups = GroupMemberRepository::list_user_groups_with_ancestors(pool, user_id).await?;

        // 获取所有组的权限
        let mut permissions = Vec::new();
        for group in &groups {
            let group_perms = GroupPermissionRepository::list_permissions(pool, group.id).await?;
            permissions.extend(group_perms);
        }

        // 去重
        permissions.sort();
        permissions.dedup();

        Ok(UserPermissions {
            user_id,
            groups,
            permissions,
        })
    }

    /// 获取用户的所有权限字符串（用于鉴权）
    pub async fn get_permission_strings(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<String>> {
        let records = sqlx::query_as::<_, (String,)>(
            r#"
            WITH RECURSIVE user_groups AS (
                -- 用户直接所在的组
                SELECT g.id, g.parent_id FROM groups g
                JOIN group_members gm ON g.id = gm.group_id
                WHERE gm.user_id = $1
            ),
            all_groups AS (
                SELECT id, parent_id FROM user_groups
                UNION
                SELECT g.id, g.parent_id FROM groups g
                JOIN all_groups ag ON g.id = ag.parent_id
            )
            SELECT DISTINCT gp.permission
            FROM group_permissions gp
            WHERE gp.group_id IN (SELECT id FROM all_groups)
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(records.into_iter().map(|(p,)| p).collect())
    }

    /// 检查用户是否拥有某权限（支持通配符匹配）
    pub async fn has_permission(
        pool: &PgPool,
        user_id: Uuid,
        required: &str,
    ) -> sqlx::Result<bool> {
        let permissions = Self::get_permission_strings(pool, user_id).await?;

        // 构造 UserPermissions 用于匹配
        let user_perms = UserPermissions {
            user_id,
            groups: vec![],
            permissions,
        };

        Ok(user_perms.has_permission(required))
    }
}
