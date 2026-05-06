use async_trait::async_trait;
use sqlx::PgPool;

use packrat_application::{TenantCommandError, TenantCommandPort};
use packrat_domain::entity::EntityTimestamp;
use packrat_domain::tenant::{Tenant, TenantId, TenantName};

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
    async fn create_tenant(&self, name: TenantName) -> Result<Tenant, TenantCommandError> {
        let trimmed = name.as_str().trim();
        let row = sqlx::query!(
            r#"
            INSERT INTO tenants (name)
            VALUES ($1)
            RETURNING id, name, created, updated
            "#,
            trimmed,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| TenantCommandError::Persist(e.to_string()))?;

        Ok(Tenant::new(
            TenantId::from(row.id),
            TenantName::from(row.name),
            EntityTimestamp::from(row.created),
            EntityTimestamp::from(row.updated),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn create_tenant_inserts_row(pool: PgPool) {
        let cmd = PostgresTenantCommand::new(pool.clone());
        let tenant = cmd
            .create_tenant(TenantName::from("  Workshop  "))
            .await
            .unwrap();
        assert_eq!(tenant.name.as_str(), "Workshop");

        let name: String = sqlx::query_scalar("SELECT name FROM tenants WHERE id = $1")
            .bind(i64::from(tenant.id))
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(name, "Workshop");
    }
}
