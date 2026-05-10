use std::collections::HashSet;

use async_trait::async_trait;
use packrat_application::{TagCommandPort, TagQueryPort};
use packrat_domain::asset::AssetId;
use packrat_domain::tag::{Tag, TagId, TagName};
use packrat_domain::tenant::TenantId;
use sqlx::PgPool;

pub struct PostgresTags {
    pool: PgPool,
}

impl PostgresTags {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagQueryPort for PostgresTags {
    async fn list_tags(&self, tenant_id: TenantId, prefix: Option<&str>) -> Vec<Tag> {
        let tid = i64::from(tenant_id);
        let normalized_prefix: Option<String> = prefix
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase());

        let rows = sqlx::query!(
            r#"SELECT id, tenant_id, name FROM tags
               WHERE tenant_id = $1
                 AND ($2::text IS NULL OR normalized LIKE $2 || '%')
               ORDER BY lower(name) ASC"#,
            tid,
            normalized_prefix as Option<String>,
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        rows.into_iter()
            .map(|row| {
                Tag::new(
                    TagId::from(row.id),
                    TenantId::from(row.tenant_id),
                    TagName::from(row.name),
                )
            })
            .collect()
    }
}

#[async_trait]
impl TagCommandPort for PostgresTags {
    async fn ensure_tag(&self, tenant_id: TenantId, name: TagName) -> Result<Tag, String> {
        let tid = i64::from(tenant_id);
        let normalized = name.normalized();
        let display = name.as_str().to_string();

        let inserted = sqlx::query!(
            r#"INSERT INTO tags (tenant_id, name, normalized)
               VALUES ($1, $2, $3)
               ON CONFLICT (tenant_id, normalized) DO NOTHING
               RETURNING id, tenant_id, name"#,
            tid,
            display,
            normalized,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = inserted {
            return Ok(Tag::new(
                TagId::from(row.id),
                TenantId::from(row.tenant_id),
                TagName::from(row.name),
            ));
        }

        let row = sqlx::query!(
            "SELECT id, tenant_id, name FROM tags WHERE tenant_id = $1 AND normalized = $2",
            tid,
            normalized,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Tag::new(
            TagId::from(row.id),
            TenantId::from(row.tenant_id),
            TagName::from(row.name),
        ))
    }

    async fn set_asset_tags(
        &self,
        tenant_id: TenantId,
        asset_id: AssetId,
        tag_ids: Vec<TagId>,
    ) -> Result<(), String> {
        let tid = i64::from(tenant_id);
        let aid = i64::from(asset_id);

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let asset_row = sqlx::query!(
            "SELECT tenant_id FROM assets WHERE id = $1 AND deleted IS NULL",
            aid,
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        match asset_row {
            Some(row) if row.tenant_id == tid => {}
            Some(_) => return Err("asset belongs to another tenant".into()),
            None => return Err("asset not found".into()),
        }

        let unique: Vec<i64> = tag_ids
            .into_iter()
            .map(i64::from)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        if unique.is_empty() {
            sqlx::query!("DELETE FROM asset_tags WHERE asset_id = $1", aid)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;
            tx.commit().await.map_err(|e| e.to_string())?;
            return Ok(());
        }

        let count: i64 = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)::bigint FROM tags WHERE tenant_id = $1 AND id = ANY($2)",
        )
        .bind(tid)
        .bind(&unique)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        if count != unique.len() as i64 {
            return Err("one or more tags not found for this tenant".into());
        }

        sqlx::query!("DELETE FROM asset_tags WHERE asset_id = $1", aid)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        for tag_id in unique {
            sqlx::query!(
                "INSERT INTO asset_tags (asset_id, tag_id) VALUES ($1, $2)",
                aid,
                tag_id,
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
