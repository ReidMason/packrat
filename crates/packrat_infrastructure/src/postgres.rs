use async_trait::async_trait;
use packrat_application::{AssetCommandPort, AssetQueryPort, AssetSearchQuery};
use packrat_domain::aggregates::partial_asset::PartialAsset;
use packrat_domain::asset::{Asset, AssetId, AssetName, AssetTimestamp};
use packrat_domain::tenant::TenantId;
use sqlx::PgPool;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;

pub struct PostgresAssetCommand {
    pool: PgPool,
}

impl PostgresAssetCommand {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AssetCommandPort for PostgresAssetCommand {
    async fn create_asset(
        &self,
        tenant_id: TenantId,
        name: AssetName,
        parent: Option<AssetId>,
    ) -> Result<Asset, String> {
        let tid = i64::from(tenant_id);
        if let Some(pid) = parent {
            let row_tid: Option<i64> = sqlx::query_scalar(
                "SELECT tenant_id FROM assets WHERE id = $1 AND deleted IS NULL",
            )
            .bind(i64::from(pid))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            match row_tid {
                Some(t) if t == tid => {}
                Some(_) => return Err("parent asset belongs to another tenant".into()),
                None => return Err("parent asset not found".into()),
            }
        }

        let created = AssetTimestamp::now();
        let deleted: Option<chrono::DateTime<chrono::Utc>> = None;
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO assets (name, parent_id, created, deleted, tenant_id) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        )
        .bind(name.as_str())
        .bind(parent.map(i64::from))
        .bind(chrono::DateTime::from(created))
        .bind(deleted)
        .bind(tid)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Asset::new(
            AssetId::from(id),
            tenant_id,
            name,
            parent,
            created,
            None,
        ))
    }

    async fn update_asset(
        &self,
        tenant_id: TenantId,
        id: AssetId,
        changes: PartialAsset,
    ) -> Result<(), String> {
        let tid = i64::from(tenant_id);
        let current_row = sqlx::query(
            "SELECT name, parent_id FROM assets WHERE id = $1 AND tenant_id = $2 AND deleted IS NULL",
        )
        .bind(i64::from(id))
        .bind(tid)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Entity {} not found", i64::from(id)))?;

        let name = match changes.name {
            Some(name) => String::from(name),
            None => current_row.try_get::<String, _>("name").map_err(|e| e.to_string())?,
        };

        let parent = match changes.parent {
            Some(new_parent) => new_parent.map(i64::from),
            None => current_row
                .try_get::<Option<i64>, _>("parent_id")
                .map_err(|e| e.to_string())?,
        };

        if let Some(pid) = parent {
            let row_tid: Option<i64> = sqlx::query_scalar(
                "SELECT tenant_id FROM assets WHERE id = $1 AND deleted IS NULL",
            )
            .bind(pid)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            match row_tid {
                Some(t) if t == tid => {}
                Some(_) => return Err("parent asset belongs to another tenant".into()),
                None => return Err("parent asset not found".into()),
            }
        }

        let result = sqlx::query("UPDATE assets SET name = $1, parent_id = $2 WHERE id = $3 AND tenant_id = $4")
            .bind(&name)
            .bind(parent)
            .bind(i64::from(id))
            .bind(tid)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            return Err(format!("Entity with ID {} not found", i64::from(id)));
        }

        Ok(())
    }

    async fn delete_asset(&self, tenant_id: TenantId, id: AssetId) -> Result<(), String> {
        let tid = i64::from(tenant_id);
        let is_a_parent: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM assets WHERE parent_id = $1 AND deleted IS NULL AND tenant_id = $2)",
        )
        .bind(i64::from(id))
        .bind(tid)
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.to_string())?;

        if is_a_parent {
            return Err("Cannot Delete: Entity has active children".into());
        }

        let result = sqlx::query(
            "UPDATE assets SET deleted = NOW() WHERE id = $1 AND tenant_id = $2 AND deleted IS NULL",
        )
        .bind(i64::from(id))
        .bind(tid)
        .execute(&self.pool)
        .await
        .map_err(|err| err.to_string())?;

        if result.rows_affected() == 0 {
            return Err(format!("Asset with ID {} not found", i64::from(id)));
        }

        Ok(())
    }
}

pub struct PostgresAssetQuery {
    pool: PgPool,
}

impl PostgresAssetQuery {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn entity_from_row(row: &sqlx::postgres::PgRow) -> Option<Asset> {
        let id: i64 = row.try_get("id").ok()?;
        let tenant_id: i64 = row.try_get("tenant_id").ok()?;
        let name: String = row.try_get("name").ok()?;
        let parent_id: Option<i64> = row.try_get("parent_id").ok()?;
        let created: chrono::DateTime<chrono::Utc> = row
            .try_get::<'_, chrono::DateTime<chrono::Utc>, _>("created")
            .ok()?;
        let deleted: Option<chrono::DateTime<chrono::Utc>> = row
            .try_get::<'_, Option<chrono::DateTime<chrono::Utc>>, _>("deleted")
            .ok()?;

        Some(Asset::new(
            AssetId::from(id),
            TenantId::from(tenant_id),
            AssetName::from(name),
            parent_id.map(AssetId::from),
            AssetTimestamp::from(created),
            deleted.map(AssetTimestamp::from),
        ))
    }
}

#[async_trait]
impl AssetQueryPort for PostgresAssetQuery {
    async fn get_asset_by_id(&self, tenant_id: TenantId, id: AssetId) -> Option<Asset> {
        let row = sqlx::query(
            "SELECT id, tenant_id, name, parent_id, created, deleted FROM assets WHERE id = $1 AND tenant_id = $2",
        )
        .bind(i64::from(id))
        .bind(i64::from(tenant_id))
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()?;

        Self::entity_from_row(&row)
    }

    async fn list_active_assets(&self, tenant_id: TenantId) -> Vec<Asset> {
        let rows = sqlx::query(
            "SELECT id, tenant_id, name, parent_id, created, deleted FROM assets WHERE deleted IS NULL AND tenant_id = $1 ORDER BY LOWER(name) ASC",
        )
        .bind(i64::from(tenant_id))
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        rows.iter()
            .filter_map(|row| Self::entity_from_row(row))
            .collect()
    }

    async fn search_assets(&self, tenant_id: TenantId, query: &AssetSearchQuery) -> Vec<Asset> {
        let name = query
            .name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        let fuzzy = query
            .fuzzyname
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        let rows = sqlx::query(
            r#"SELECT id, tenant_id, name, parent_id, created, deleted FROM assets
               WHERE deleted IS NULL AND tenant_id = $1
                 AND ($2::text IS NULL OR name = $2)
                 AND ($3::text IS NULL OR strpos(lower(name), lower($3)) > 0)
               ORDER BY LOWER(name) ASC"#,
        )
        .bind(i64::from(tenant_id))
        .bind(name)
        .bind(fuzzy)
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        rows.iter()
            .filter_map(|row| Self::entity_from_row(row))
            .collect()
    }

    async fn list_child_assets(&self, tenant_id: TenantId, parent_id: AssetId) -> Vec<Asset> {
        let rows = sqlx::query(
            "SELECT id, tenant_id, name, parent_id, created, deleted FROM assets WHERE deleted IS NULL AND tenant_id = $1 AND parent_id = $2 ORDER BY LOWER(name) ASC",
        )
        .bind(i64::from(tenant_id))
        .bind(i64::from(parent_id))
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        rows.iter()
            .filter_map(|row| Self::entity_from_row(row))
            .collect()
    }
}

pub async fn connect_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

pub async fn ping_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod postgres_tests {
    use super::*;
    use packrat_domain::tenant::TenantId;
    use sqlx::Row;

    async fn insert_test_tenant(pool: &PgPool) -> TenantId {
        let id: i64 = sqlx::query_scalar("INSERT INTO tenants (name) VALUES ($1) RETURNING id")
            .bind("postgres asset tests")
            .fetch_one(pool)
            .await
            .unwrap();
        TenantId::from(id)
    }

    #[sqlx::test]
    async fn test_delete_asset_errors_when_is_parent(pool: PgPool) {
        let tenant_id = insert_test_tenant(&pool).await;
        let command = PostgresAssetCommand::new(pool.clone());

        let parent = command
            .create_asset(tenant_id, AssetName::from("Parent"), None)
            .await
            .unwrap();

        let _child = command
            .create_asset(tenant_id, AssetName::from("Child"), Some(parent.id))
            .await
            .unwrap();

        let result = command.delete_asset(tenant_id, parent.id).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cannot Delete: Entity has active children"
        );

        let row = sqlx::query("SELECT deleted FROM assets WHERE id = $1")
            .bind(i64::from(parent.id))
            .fetch_one(&pool)
            .await
            .unwrap();
        let deleted: Option<chrono::DateTime<chrono::Utc>> = row.try_get("deleted").unwrap();

        assert!(deleted.is_none());
    }

    #[sqlx::test]
    async fn test_delete_non_existent_asset_returns_error(pool: PgPool) {
        let tenant_id = insert_test_tenant(&pool).await;
        let command = PostgresAssetCommand::new(pool);
        let fake_id = AssetId::from(999);

        let result = command.delete_asset(tenant_id, fake_id).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[sqlx::test]
    async fn test_delete_asset_successfully(pool: PgPool) {
        let tenant_id = insert_test_tenant(&pool).await;
        let command = PostgresAssetCommand::new(pool.clone());
        let asset = command
            .create_asset(tenant_id, AssetName::from("Target"), None)
            .await
            .unwrap();

        let result = command.delete_asset(tenant_id, asset.id).await;

        assert!(result.is_ok());

        let row = sqlx::query("SELECT deleted FROM assets WHERE id = $1")
            .bind(i64::from(asset.id))
            .fetch_one(&pool)
            .await
            .unwrap();
        let deleted: Option<chrono::DateTime<chrono::Utc>> = row.try_get("deleted").unwrap();

        assert!(deleted.is_some());
    }

    #[sqlx::test]
    async fn test_update_asset_name_only(pool: PgPool) {
        let tenant_id = insert_test_tenant(&pool).await;
        let command = PostgresAssetCommand::new(pool.clone());
        let asset = command
            .create_asset(tenant_id, AssetName::from("Old Name"), None)
            .await
            .unwrap();

        let changes = PartialAsset {
            name: Some(AssetName::from("New Name")),
            parent: None,
        };

        let result = command.update_asset(tenant_id, asset.id, changes).await;
        assert!(result.is_ok());

        let row = sqlx::query("SELECT name FROM assets WHERE id = $1")
            .bind(i64::from(asset.id))
            .fetch_one(&pool)
            .await
            .unwrap();
        let name: String = row.try_get("name").unwrap();

        assert_eq!(name, "New Name");
    }
}
