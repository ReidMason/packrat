use async_trait::async_trait;
use sqlx::PgPool;

use packrat_application::{TenantCommandError, TenantCommandPort, TenantMembershipQueryPort};
use packrat_domain::asset::AssetTimestamp;
use packrat_domain::tenant::{Tenant, TenantId, TenantName};
use packrat_domain::user::UserId;

pub struct PostgresTenantCommand {
    pool: PgPool,
}

impl PostgresTenantCommand {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantCommandPort for PostgresTenantCommand {
    async fn create_tenant(
        &self,
        user_id: UserId,
        name: TenantName,
    ) -> Result<Tenant, TenantCommandError> {
        let trimmed = name.as_str().trim();
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        let row = sqlx::query!(
            r#"
            INSERT INTO tenants (name)
            VALUES ($1)
            RETURNING id, name, created, updated
            "#,
            trimmed,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        let owner_role_id = sqlx::query_scalar!(
            "SELECT id FROM roles WHERE name = 'Owner' AND tenant_id IS NULL LIMIT 1"
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id, tenant_id) VALUES ($1, $2, $3)",
            i64::from(user_id),
            owner_role_id,
            row.id,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        Ok(Tenant::new(
            TenantId::from(row.id),
            TenantName::from(row.name),
            AssetTimestamp::from(row.created),
            AssetTimestamp::from(row.updated),
        ))
    }
}

pub struct PostgresTenantQuery {
    pool: PgPool,
}

impl PostgresTenantQuery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantMembershipQueryPort for PostgresTenantQuery {
    async fn list_tenants_for_user(&self, user_id: UserId) -> Result<Vec<Tenant>, String> {
        let rows = sqlx::query!(
            r#"
            SELECT t.id, t.name, t.created, t.updated
            FROM tenants t
            WHERE t.id IN (
                SELECT DISTINCT ur.tenant_id
                FROM user_roles ur
                WHERE ur.user_id = $1
            )
            ORDER BY LOWER(t.name) ASC
            "#,
            i64::from(user_id),
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            out.push(Tenant::new(
                TenantId::from(row.id),
                TenantName::from(row.name),
                AssetTimestamp::from(row.created),
                AssetTimestamp::from(row.updated),
            ));
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn template_role_id(pool: &PgPool, name: &str) -> i64 {
        sqlx::query_scalar!(
            "SELECT id FROM roles WHERE name = $1 AND tenant_id IS NULL LIMIT 1",
            name,
        )
        .fetch_one(pool)
        .await
        .unwrap()
    }

    async fn insert_user(pool: &PgPool, email: &str) -> i64 {
        sqlx::query_scalar!(
            "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING id",
            email,
            "x",
        )
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[sqlx::test]
    async fn list_tenants_for_user_returns_only_memberships(pool: PgPool) {
        let uid_a = insert_user(&pool, "a-list-tenants@example.com").await;
        let uid_b = insert_user(&pool, "b-list-tenants@example.com").await;

        let tid_alpha = sqlx::query_scalar!(
            "INSERT INTO tenants (name) VALUES ($1) RETURNING id",
            "Alpha WS",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        let tid_beta = sqlx::query_scalar!(
            "INSERT INTO tenants (name) VALUES ($1) RETURNING id",
            "Beta WS",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let rid_owner = template_role_id(&pool, "Owner").await;
        let rid_viewer = template_role_id(&pool, "Viewer").await;

        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id, tenant_id) VALUES ($1, $2, $3), ($1, $4, $5)",
            uid_a,
            rid_owner,
            tid_alpha,
            rid_viewer,
            tid_beta,
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id, tenant_id) VALUES ($1, $2, $3)",
            uid_b,
            rid_viewer,
            tid_beta,
        )
        .execute(&pool)
        .await
        .unwrap();

        let query = PostgresTenantQuery::new(pool.clone());
        let for_a = query
            .list_tenants_for_user(UserId::from(uid_a))
            .await
            .unwrap();
        let for_b = query
            .list_tenants_for_user(UserId::from(uid_b))
            .await
            .unwrap();

        assert_eq!(for_a.len(), 2);
        assert_eq!(for_b.len(), 1);
        assert_eq!(i64::from(for_b[0].id), tid_beta);

        let names_a: Vec<&str> = for_a.iter().map(|t| t.name.as_str()).collect();
        assert!(names_a.contains(&"Alpha WS"));
        assert!(names_a.contains(&"Beta WS"));
    }

    #[sqlx::test]
    async fn create_tenant_inserts_membership_for_creator(pool: PgPool) {
        let cmd = PostgresTenantCommand::new(pool.clone());
        let uid = insert_user(&pool, "creator-tenant@example.com").await;

        let tenant = cmd
            .create_tenant(UserId::from(uid), TenantName::from("  Workshop  "))
            .await
            .unwrap();
        assert_eq!(tenant.name.as_str(), "Workshop");

        let owner_role_id = template_role_id(&pool, "Owner").await;
        let cnt = sqlx::query_scalar!(
            r#"SELECT COUNT(*)::bigint AS "count!" FROM user_roles WHERE user_id = $1 AND tenant_id = $2 AND role_id = $3"#,
            uid,
            i64::from(tenant.id),
            owner_role_id,
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(cnt, 1);
    }
}
