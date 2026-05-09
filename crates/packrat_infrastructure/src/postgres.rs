use std::collections::HashMap;

use async_trait::async_trait;
use packrat_application::{AssetCommandPort, AssetQueryPort, AssetSearchQuery, AssetWithTags};
use packrat_domain::aggregates::partial_asset::PartialAsset;
use packrat_domain::asset::{Asset, AssetId, AssetName, AssetTimestamp};
use packrat_domain::tag::{Tag, TagId, TagName};
use packrat_domain::tenant::TenantId;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

fn asset_from_db(
    id: i64,
    tenant_id: i64,
    name: String,
    parent_id: Option<i64>,
    created: chrono::DateTime<chrono::Utc>,
    deleted: Option<chrono::DateTime<chrono::Utc>>,
) -> Asset {
    Asset::new(
        AssetId::from(id),
        TenantId::from(tenant_id),
        AssetName::from(name),
        parent_id.map(AssetId::from),
        AssetTimestamp::from(created),
        deleted.map(AssetTimestamp::from),
    )
}

fn tag_from_db(id: i64, tenant_id: i64, name: String) -> Tag {
    Tag::new(
        TagId::from(id),
        TenantId::from(tenant_id),
        TagName::from(name),
    )
}

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
            let row_tid: Option<i64> = sqlx::query_scalar!(
                "SELECT tenant_id FROM assets WHERE id = $1 AND deleted IS NULL",
                i64::from(pid),
            )
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
        let id: i64 = sqlx::query_scalar!(
            "INSERT INTO assets (name, parent_id, created, deleted, tenant_id) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            name.as_str(),
            parent.map(i64::from),
            chrono::DateTime::from(created),
            deleted,
            tid,
        )
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
        let current_row = sqlx::query!(
            "SELECT name, parent_id FROM assets WHERE id = $1 AND tenant_id = $2 AND deleted IS NULL",
            i64::from(id),
            tid,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Entity {} not found", i64::from(id)))?;

        let name = match changes.name {
            Some(name) => String::from(name),
            None => current_row.name,
        };

        let parent = match changes.parent {
            Some(new_parent) => new_parent.map(i64::from),
            None => current_row.parent_id,
        };

        if let Some(pid) = parent {
            let row_tid: Option<i64> = sqlx::query_scalar!(
                "SELECT tenant_id FROM assets WHERE id = $1 AND deleted IS NULL",
                pid,
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
            match row_tid {
                Some(t) if t == tid => {}
                Some(_) => return Err("parent asset belongs to another tenant".into()),
                None => return Err("parent asset not found".into()),
            }
        }

        let result = sqlx::query!(
            "UPDATE assets SET name = $1, parent_id = $2 WHERE id = $3 AND tenant_id = $4",
            name,
            parent,
            i64::from(id),
            tid,
        )
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
        let is_a_parent: bool = sqlx::query_scalar!(
            "SELECT EXISTS (SELECT 1 FROM assets WHERE parent_id = $1 AND deleted IS NULL AND tenant_id = $2) as \"exists!\"",
            i64::from(id),
            tid,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.to_string())?;

        if is_a_parent {
            return Err("Cannot Delete: Entity has active children".into());
        }

        let result = sqlx::query!(
            "UPDATE assets SET deleted = NOW() WHERE id = $1 AND tenant_id = $2 AND deleted IS NULL",
            i64::from(id),
            tid,
        )
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

    async fn load_tags_for_assets(
        &self,
        tenant_id: TenantId,
        asset_ids: &[i64],
    ) -> HashMap<i64, Vec<Tag>> {
        if asset_ids.is_empty() {
            return HashMap::new();
        }
        let tid = i64::from(tenant_id);
        let rows = sqlx::query!(
            r#"SELECT at.asset_id, t.id as "tag_id!", t.tenant_id, t.name
               FROM asset_tags at
               INNER JOIN tags t ON t.id = at.tag_id
               WHERE at.asset_id = ANY($1) AND t.tenant_id = $2
               ORDER BY at.asset_id ASC, lower(t.name) ASC"#,
            asset_ids as &[i64],
            tid,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let mut map: HashMap<i64, Vec<Tag>> = HashMap::new();
        for row in rows {
            map.entry(row.asset_id).or_default().push(tag_from_db(
                row.tag_id,
                row.tenant_id,
                row.name,
            ));
        }
        map
    }

    async fn merge_with_tags(&self, tenant_id: TenantId, assets: Vec<Asset>) -> Vec<AssetWithTags> {
        let ids: Vec<i64> = assets.iter().map(|a| i64::from(a.id)).collect();
        let map = self.load_tags_for_assets(tenant_id, &ids).await;
        assets
            .into_iter()
            .map(|a| {
                let id = i64::from(a.id);
                let tags = map.get(&id).cloned().unwrap_or_default();
                AssetWithTags::new(a, tags)
            })
            .collect()
    }
}

#[async_trait]
impl AssetQueryPort for PostgresAssetQuery {
    async fn get_asset_by_id(&self, tenant_id: TenantId, id: AssetId) -> Option<AssetWithTags> {
        let row = sqlx::query!(
            "SELECT id, tenant_id, name, parent_id, created, deleted FROM assets WHERE id = $1 AND tenant_id = $2",
            i64::from(id),
            i64::from(tenant_id),
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()?;

        let asset = asset_from_db(
            row.id,
            row.tenant_id,
            row.name,
            row.parent_id,
            row.created,
            row.deleted,
        );
        let map = self.load_tags_for_assets(tenant_id, &[row.id]).await;
        let tags = map.get(&row.id).cloned().unwrap_or_default();
        Some(AssetWithTags::new(asset, tags))
    }

    async fn list_active_assets(&self, tenant_id: TenantId) -> Vec<AssetWithTags> {
        let rows = sqlx::query!(
            "SELECT id, tenant_id, name, parent_id, created, deleted FROM assets WHERE deleted IS NULL AND tenant_id = $1 ORDER BY LOWER(name) ASC",
            i64::from(tenant_id),
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let assets: Vec<Asset> = rows
            .into_iter()
            .map(|row| {
                asset_from_db(
                    row.id,
                    row.tenant_id,
                    row.name,
                    row.parent_id,
                    row.created,
                    row.deleted,
                )
            })
            .collect();

        self.merge_with_tags(tenant_id, assets).await
    }

    async fn search_assets(
        &self,
        tenant_id: TenantId,
        query: &AssetSearchQuery,
    ) -> Vec<AssetWithTags> {
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

        let rows = sqlx::query!(
            r#"SELECT id, tenant_id, name, parent_id, created, deleted FROM assets
               WHERE deleted IS NULL AND tenant_id = $1
                 AND ($2::text IS NULL OR name = $2)
                 AND ($3::text IS NULL OR strpos(lower(name), lower($3)) > 0)
               ORDER BY LOWER(name) ASC"#,
            i64::from(tenant_id),
            name as Option<String>,
            fuzzy as Option<String>,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let assets: Vec<Asset> = rows
            .into_iter()
            .map(|row| {
                asset_from_db(
                    row.id,
                    row.tenant_id,
                    row.name,
                    row.parent_id,
                    row.created,
                    row.deleted,
                )
            })
            .collect();

        self.merge_with_tags(tenant_id, assets).await
    }

    async fn list_child_assets(
        &self,
        tenant_id: TenantId,
        parent_id: AssetId,
    ) -> Vec<AssetWithTags> {
        let rows = sqlx::query!(
            "SELECT id, tenant_id, name, parent_id, created, deleted FROM assets WHERE deleted IS NULL AND tenant_id = $1 AND parent_id = $2 ORDER BY LOWER(name) ASC",
            i64::from(tenant_id),
            i64::from(parent_id),
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        let assets: Vec<Asset> = rows
            .into_iter()
            .map(|row| {
                asset_from_db(
                    row.id,
                    row.tenant_id,
                    row.name,
                    row.parent_id,
                    row.created,
                    row.deleted,
                )
            })
            .collect();

        self.merge_with_tags(tenant_id, assets).await
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
    sqlx::query_scalar!(r#"SELECT 1 as "one!: i32""#)
        .fetch_one(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod postgres_tests {
    use super::*;
    use crate::postgres_tags::PostgresTags;
    use packrat_application::{AssetQueryPort, TagCommandPort, TagQueryPort};
    use packrat_domain::tag::TagName;
    use packrat_domain::tenant::TenantId;

    async fn insert_test_tenant(pool: &PgPool) -> TenantId {
        let id: i64 = sqlx::query_scalar!(
            "INSERT INTO tenants (name) VALUES ($1) RETURNING id",
            "postgres asset tests",
        )
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

        let row = sqlx::query!(
            "SELECT deleted FROM assets WHERE id = $1",
            i64::from(parent.id),
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert!(row.deleted.is_none());
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

        let row = sqlx::query!(
            "SELECT deleted FROM assets WHERE id = $1",
            i64::from(asset.id),
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert!(row.deleted.is_some());
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

        let row = sqlx::query!("SELECT name FROM assets WHERE id = $1", i64::from(asset.id),)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.name, "New Name");
    }

    #[sqlx::test]
    async fn ensure_tag_normalizes_case(pool: PgPool) {
        let tenant_id = insert_test_tenant(&pool).await;
        let tags = PostgresTags::new(pool);
        let first = tags
            .ensure_tag(tenant_id, TagName::parse("Electronics").unwrap())
            .await
            .unwrap();
        let second = tags
            .ensure_tag(tenant_id, TagName::parse("electronics").unwrap())
            .await
            .unwrap();
        assert_eq!(first.id, second.id);
        assert_eq!(first.name.as_str(), "Electronics");
    }

    #[sqlx::test]
    async fn set_asset_tags_rejects_tag_from_other_tenant(pool: PgPool) {
        let tenant_a = insert_test_tenant(&pool).await;
        let tenant_b = insert_test_tenant(&pool).await;
        let command = PostgresAssetCommand::new(pool.clone());
        let tags = PostgresTags::new(pool.clone());
        let asset = command
            .create_asset(tenant_a, AssetName::from("Thing"), None)
            .await
            .unwrap();
        let foreign = tags
            .ensure_tag(tenant_b, TagName::parse("orphan").unwrap())
            .await
            .unwrap();
        let err = tags
            .set_asset_tags(tenant_a, asset.id, vec![foreign.id])
            .await
            .unwrap_err();
        assert!(err.contains("not found"));
    }

    #[sqlx::test]
    async fn asset_queries_include_joined_tags(pool: PgPool) {
        let tenant_id = insert_test_tenant(&pool).await;
        let command = PostgresAssetCommand::new(pool.clone());
        let tags = PostgresTags::new(pool.clone());
        let asset = command
            .create_asset(tenant_id, AssetName::from("Camera"), None)
            .await
            .unwrap();
        let tag = tags
            .ensure_tag(tenant_id, TagName::parse("photo").unwrap())
            .await
            .unwrap();
        tags.set_asset_tags(tenant_id, asset.id, vec![tag.id])
            .await
            .unwrap();

        let query = PostgresAssetQuery::new(pool.clone());
        let by_id = query.get_asset_by_id(tenant_id, asset.id).await.unwrap();
        assert_eq!(by_id.tags.len(), 1);
        assert_eq!(by_id.tags[0].name.as_str(), "photo");

        let listed = query.list_active_assets(tenant_id).await;
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].tags.len(), 1);
    }

    #[sqlx::test]
    async fn list_tags_prefix_matches_normalized(pool: PgPool) {
        let tenant_id = insert_test_tenant(&pool).await;
        let tags = PostgresTags::new(pool);
        tags.ensure_tag(tenant_id, TagName::parse("apple").unwrap())
            .await
            .unwrap();
        tags.ensure_tag(tenant_id, TagName::parse("application").unwrap())
            .await
            .unwrap();
        tags.ensure_tag(tenant_id, TagName::parse("banana").unwrap())
            .await
            .unwrap();
        let apple_prefixed = tags.list_tags(tenant_id, Some("app")).await;
        assert_eq!(apple_prefixed.len(), 2);
        let all = tags.list_tags(tenant_id, None).await;
        assert_eq!(all.len(), 3);
    }
}
