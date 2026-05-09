use async_trait::async_trait;
use sqlx::{PgPool, Row};

use packrat_application::{
    TenantCommandError, TenantCommandPort, TenantMembershipQueryPort,
};
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

        let row = sqlx::query(
            r#"
            INSERT INTO tenants (name)
            VALUES ($1)
            RETURNING id, name, created, updated
            "#,
        )
        .bind(trimmed)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        let id: i64 = row.try_get("id").map_err(|e| TenantCommandError::Persist(e.to_string()))?;
        let name_db: String = row
            .try_get("name")
            .map_err(|e| TenantCommandError::Persist(e.to_string()))?;
        let created: chrono::DateTime<chrono::Utc> = row
            .try_get("created")
            .map_err(|e| TenantCommandError::Persist(e.to_string()))?;
        let updated: chrono::DateTime<chrono::Utc> = row
            .try_get("updated")
            .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        sqlx::query("INSERT INTO user_tenants (user_id, tenant_id) VALUES ($1, $2)")
        .bind(i64::from(user_id))
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        Ok(Tenant::new(
            TenantId::from(id),
            TenantName::from(name_db),
            AssetTimestamp::from(created),
            AssetTimestamp::from(updated),
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
        let rows = sqlx::query(
            r#"
            SELECT t.id, t.name, t.created, t.updated
            FROM tenants t
            INNER JOIN user_tenants ut ON ut.tenant_id = t.id
            WHERE ut.user_id = $1
            ORDER BY LOWER(t.name) ASC
            "#,
        )
        .bind(i64::from(user_id))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let id: i64 = row.try_get("id").map_err(|e| e.to_string())?;
            let name: String = row.try_get("name").map_err(|e| e.to_string())?;
            let created: chrono::DateTime<chrono::Utc> =
                row.try_get("created").map_err(|e| e.to_string())?;
            let updated: chrono::DateTime<chrono::Utc> =
                row.try_get("updated").map_err(|e| e.to_string())?;
            out.push(Tenant::new(
                TenantId::from(id),
                TenantName::from(name),
                AssetTimestamp::from(created),
                AssetTimestamp::from(updated),
            ));
        }
        Ok(out)
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

    #[sqlx::test]
    async fn list_tenants_for_user_returns_only_memberships(pool: PgPool) {
        let uid_a = insert_user(&pool, "a-list-tenants@example.com").await;
        let uid_b = insert_user(&pool, "b-list-tenants@example.com").await;

        let tid_alpha: i64 = sqlx::query_scalar("INSERT INTO tenants (name) VALUES ($1) RETURNING id")
            .bind("Alpha WS")
            .fetch_one(&pool)
            .await
            .unwrap();
        let tid_beta: i64 = sqlx::query_scalar("INSERT INTO tenants (name) VALUES ($1) RETURNING id")
            .bind("Beta WS")
            .fetch_one(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO user_tenants (user_id, tenant_id) VALUES ($1, $2), ($1, $3)")
            .bind(uid_a)
            .bind(tid_alpha)
            .bind(tid_beta)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO user_tenants (user_id, tenant_id) VALUES ($1, $2)")
            .bind(uid_b)
            .bind(tid_beta)
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

        let cnt: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::bigint FROM user_tenants WHERE user_id = $1 AND tenant_id = $2",
        )
        .bind(uid)
        .bind(i64::from(tenant.id))
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(cnt, 1);
    }
}
