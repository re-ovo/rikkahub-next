//! 身份组数据访问

use crate::entities::*;
use sqlx::PgPool;
use uuid::Uuid;

/// 身份组仓库
pub struct GroupRepository;

impl GroupRepository {
    /// 根据 ID 查询组
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<Group>> {
        sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// 根据 code 查询组
    pub async fn find_by_code(pool: &PgPool, code: &str) -> sqlx::Result<Option<Group>> {
        sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE code = $1")
            .bind(code)
            .fetch_optional(pool)
            .await
    }

    /// 查询所有组
    pub async fn list(pool: &PgPool) -> sqlx::Result<Vec<Group>> {
        sqlx::query_as::<_, Group>("SELECT * FROM groups ORDER BY created_at")
            .fetch_all(pool)
            .await
    }

    /// 查询顶级组（无父组）
    pub async fn list_root(pool: &PgPool) -> sqlx::Result<Vec<Group>> {
        sqlx::query_as::<_, Group>(
            "SELECT * FROM groups WHERE parent_id IS NULL ORDER BY created_at",
        )
        .fetch_all(pool)
        .await
    }

    /// 查询子组
    pub async fn list_children(pool: &PgPool, parent_id: Uuid) -> sqlx::Result<Vec<Group>> {
        sqlx::query_as::<_, Group>("SELECT * FROM groups WHERE parent_id = $1 ORDER BY created_at")
            .bind(parent_id)
            .fetch_all(pool)
            .await
    }

    /// 查询组及其所有祖先组
    pub async fn get_with_ancestors(pool: &PgPool, id: Uuid) -> sqlx::Result<Vec<Group>> {
        sqlx::query_as::<_, Group>(
            r#"
            WITH RECURSIVE ancestors AS (
                SELECT * FROM groups WHERE id = $1
                UNION ALL
                SELECT g.* FROM groups g
                JOIN ancestors a ON g.id = a.parent_id
            )
            SELECT * FROM ancestors
            "#,
        )
        .bind(id)
        .fetch_all(pool)
        .await
    }

    /// 创建组
    pub async fn create(pool: &PgPool, req: CreateGroup) -> sqlx::Result<Group> {
        sqlx::query_as::<_, Group>(
            r#"
            INSERT INTO groups (code, name, description, parent_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&req.code)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.parent_id)
        .fetch_one(pool)
        .await
    }

    /// 更新组
    pub async fn update(pool: &PgPool, id: Uuid, req: UpdateGroup) -> sqlx::Result<Option<Group>> {
        sqlx::query_as::<_, Group>(
            r#"
            UPDATE groups SET
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                parent_id = COALESCE($4, parent_id)
            WHERE id = $1 AND is_system = FALSE
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.parent_id)
        .fetch_optional(pool)
        .await
    }

    /// 删除组（系统组不可删除）
    pub async fn delete(pool: &PgPool, id: Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query("DELETE FROM groups WHERE id = $1 AND is_system = FALSE")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 构建组树
    pub async fn build_tree(pool: &PgPool) -> sqlx::Result<Vec<GroupTreeNode>> {
        let groups = Self::list(pool).await?;
        Ok(build_tree_recursive(&groups, None))
    }
}

/// 递归构建树
fn build_tree_recursive(groups: &[Group], parent_id: Option<Uuid>) -> Vec<GroupTreeNode> {
    groups
        .iter()
        .filter(|g| g.parent_id == parent_id)
        .map(|g| GroupTreeNode {
            group: g.clone(),
            children: build_tree_recursive(groups, Some(g.id)),
        })
        .collect()
}

/// 组成员仓库
pub struct GroupMemberRepository;

impl GroupMemberRepository {
    /// 查询组的成员
    pub async fn list_members(pool: &PgPool, group_id: Uuid) -> sqlx::Result<Vec<GroupMember>> {
        sqlx::query_as::<_, GroupMember>("SELECT * FROM group_members WHERE group_id = $1")
            .bind(group_id)
            .fetch_all(pool)
            .await
    }

    /// 查询用户所在的组
    pub async fn list_user_groups(pool: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<Group>> {
        sqlx::query_as::<_, Group>(
            r#"
            SELECT g.* FROM groups g
            JOIN group_members gm ON g.id = gm.group_id
            WHERE gm.user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// 查询用户所在的组及其祖先组
    pub async fn list_user_groups_with_ancestors(
        pool: &PgPool,
        user_id: Uuid,
    ) -> sqlx::Result<Vec<Group>> {
        sqlx::query_as::<_, Group>(
            r#"
            WITH RECURSIVE user_groups AS (
                -- 用户直接所在的组
                SELECT g.* FROM groups g
                JOIN group_members gm ON g.id = gm.group_id
                WHERE gm.user_id = $1
            ),
            all_groups AS (
                SELECT * FROM user_groups
                UNION
                SELECT g.* FROM groups g
                JOIN all_groups ag ON g.id = ag.parent_id
            )
            SELECT DISTINCT * FROM all_groups
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// 添加成员
    pub async fn add(pool: &PgPool, req: AddGroupMember) -> sqlx::Result<GroupMember> {
        sqlx::query_as::<_, GroupMember>(
            r#"
            INSERT INTO group_members (group_id, user_id)
            VALUES ($1, $2)
            RETURNING *
            "#,
        )
        .bind(req.group_id)
        .bind(req.user_id)
        .fetch_one(pool)
        .await
    }

    /// 移除成员
    pub async fn remove(pool: &PgPool, group_id: Uuid, user_id: Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query("DELETE FROM group_members WHERE group_id = $1 AND user_id = $2")
            .bind(group_id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 检查用户是否是组成员
    pub async fn is_member(pool: &PgPool, group_id: Uuid, user_id: Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM group_members WHERE group_id = $1 AND user_id = $2",
        )
        .bind(group_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(result.0 > 0)
    }
}

/// 组权限仓库
pub struct GroupPermissionRepository;

impl GroupPermissionRepository {
    /// 查询组的权限
    pub async fn list_permissions(pool: &PgPool, group_id: Uuid) -> sqlx::Result<Vec<String>> {
        let records = sqlx::query_as::<_, (String,)>(
            "SELECT permission FROM group_permissions WHERE group_id = $1",
        )
        .bind(group_id)
        .fetch_all(pool)
        .await?;
        Ok(records.into_iter().map(|(p,)| p).collect())
    }

    /// 添加权限
    pub async fn add(pool: &PgPool, group_id: Uuid, permission: &str) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO group_permissions (group_id, permission) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(group_id)
        .bind(permission)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// 移除权限
    pub async fn remove(pool: &PgPool, group_id: Uuid, permission: &str) -> sqlx::Result<bool> {
        let result =
            sqlx::query("DELETE FROM group_permissions WHERE group_id = $1 AND permission = $2")
                .bind(group_id)
                .bind(permission)
                .execute(pool)
                .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 设置组权限（替换所有）
    pub async fn set_permissions(
        pool: &PgPool,
        group_id: Uuid,
        permissions: &[String],
    ) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM group_permissions WHERE group_id = $1")
            .bind(group_id)
            .execute(pool)
            .await?;

        for permission in permissions {
            sqlx::query("INSERT INTO group_permissions (group_id, permission) VALUES ($1, $2)")
                .bind(group_id)
                .bind(permission)
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}
