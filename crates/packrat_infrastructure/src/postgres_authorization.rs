use async_trait::async_trait;
use sqlx::PgPool;

use packrat_application::AuthorizationQueryPort;
use packrat_domain::tenant::TenantId;
use packrat_domain::user::UserId;
use packrat_domain::PermissionSlug;

pub struct PostgresAuthorizationQuery {
    pool: PgPool,
}

impl PostgresAuthorizationQuery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthorizationQueryPort for PostgresAuthorizationQuery {
    async fn user_has_permission(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        slug: PermissionSlug,
    ) -> Result<bool, String> {
        let slug_str = slug.as_str();
        let row: bool = sqlx::query_scalar(
            r#"
            WITH RECURSIVE chain AS (
                SELECT id, parent_id
                FROM permissions
                WHERE slug = $3
                UNION ALL
                SELECT p.id, p.parent_id
                FROM permissions p
                INNER JOIN chain ch ON p.id = ch.parent_id
            )
            SELECT EXISTS (
                SELECT 1
                FROM user_roles ur
                INNER JOIN role_permissions rp ON rp.role_id = ur.role_id
                INNER JOIN chain c ON c.id = rp.permission_id
                WHERE ur.user_id = $1
                  AND ur.tenant_id = $2
            )
            OR EXISTS (
                SELECT 1
                FROM user_permissions up
                INNER JOIN chain c ON c.id = up.permission_id
                WHERE up.user_id = $1
                  AND up.tenant_id = $2
                  AND (up.expires IS NULL OR up.expires > NOW())
            )
            "#,
        )
        .bind(i64::from(user_id))
        .bind(i64::from(tenant_id))
        .bind(slug_str)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn insert_user(pool: &PgPool, email: &str) -> i64 {
        sqlx::query_scalar(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
        )
        .bind(email)
        .bind("x")
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn template_role_id(pool: &PgPool, name: &str) -> i64 {
        sqlx::query_scalar(
            "SELECT id FROM roles WHERE name = $1 AND tenant_id IS NULL LIMIT 1",
        )
        .bind(name)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[sqlx::test]
    async fn viewer_has_assets_read_not_write(pool: PgPool) {
        let uid = insert_user(&pool, "authz-viewer@example.com").await;
        let tid: i64 = sqlx::query_scalar("INSERT INTO tenants (name) VALUES ($1) RETURNING id")
            .bind("Authz Tenant")
            .fetch_one(&pool)
            .await
            .unwrap();
        let rid = template_role_id(&pool, "Viewer").await;
        sqlx::query("INSERT INTO user_roles (user_id, role_id, tenant_id) VALUES ($1, $2, $3)")
            .bind(uid)
            .bind(rid)
            .bind(tid)
            .execute(&pool)
            .await
            .unwrap();

        let q = PostgresAuthorizationQuery::new(pool.clone());
        assert!(
            q.user_has_permission(UserId::from(uid), TenantId::from(tid), PermissionSlug::AssetsRead)
                .await
                .unwrap()
        );
        assert!(
            !q.user_has_permission(UserId::from(uid), TenantId::from(tid), PermissionSlug::AssetsWrite)
                .await
                .unwrap()
        );
        assert!(
            !q.user_has_permission(UserId::from(uid), TenantId::from(tid), PermissionSlug::AssetsDelete)
                .await
                .unwrap()
        );
    }

    #[sqlx::test]
    async fn owner_has_assets_delete(pool: PgPool) {
        let uid = insert_user(&pool, "authz-owner@example.com").await;
        let tid: i64 = sqlx::query_scalar("INSERT INTO tenants (name) VALUES ($1) RETURNING id")
            .bind("Owner Tenant")
            .fetch_one(&pool)
            .await
            .unwrap();
        let rid = template_role_id(&pool, "Owner").await;
        sqlx::query("INSERT INTO user_roles (user_id, role_id, tenant_id) VALUES ($1, $2, $3)")
            .bind(uid)
            .bind(rid)
            .bind(tid)
            .execute(&pool)
            .await
            .unwrap();

        let q = PostgresAuthorizationQuery::new(pool.clone());
        assert!(
            q.user_has_permission(UserId::from(uid), TenantId::from(tid), PermissionSlug::AssetsDelete)
                .await
                .unwrap()
        );
        assert!(
            q.user_has_permission(UserId::from(uid), TenantId::from(tid), PermissionSlug::AssetsWrite)
                .await
                .unwrap()
        );
    }

    #[sqlx::test]
    async fn user_without_role_has_no_permission(pool: PgPool) {
        let uid = insert_user(&pool, "authz-norole@example.com").await;
        let tid: i64 = sqlx::query_scalar("INSERT INTO tenants (name) VALUES ($1) RETURNING id")
            .bind("Lonely Tenant")
            .fetch_one(&pool)
            .await
            .unwrap();

        let q = PostgresAuthorizationQuery::new(pool.clone());
        assert!(
            !q.user_has_permission(UserId::from(uid), TenantId::from(tid), PermissionSlug::AssetsRead)
                .await
                .unwrap()
        );
    }
}
