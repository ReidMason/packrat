//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

mod postgres;
mod postgres_tenant;
mod postgres_user;
mod postgres_user_session;
mod readiness;

use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{
        Mutex,
        atomic::{AtomicI64, Ordering},
    },
};

pub use postgres::{
    PostgresAssetCommand, PostgresAssetQuery, connect_pool, ping_database, run_migrations,
};
pub use postgres_tenant::PostgresTenantCommand;
pub use postgres_user::PostgresUserCommand;
pub use postgres_user_session::PostgresUserSessionCommand;
pub use readiness::PostgresReadiness;

use packrat_application::{AssetCommandPort, AssetQueryPort, AssetSearchQuery};
use packrat_domain::{
    asset::{Asset, AssetId, AssetName, AssetTimestamp},
    aggregates::partial_asset::PartialAsset,
};

fn stub_entity(id: AssetId) -> Asset {
    Asset::new(
        id,
        AssetName::from("from infrastructure stub"),
        Some(AssetId::from(1)),
        AssetTimestamp::now(),
        None,
    )
}

/// Placeholder “database” for wiring demos and tests.
pub struct StubAssetQuery;

#[async_trait]
impl AssetQueryPort for StubAssetQuery {
    async fn get_asset_by_id(&self, id: AssetId) -> Option<Asset> {
        if id == AssetId::from(1) {
            Some(stub_entity(id))
        } else {
            None
        }
    }

    async fn list_active_assets(&self) -> Vec<Asset> {
        vec![stub_entity(AssetId::from(1))]
    }

    async fn search_assets(&self, query: &AssetSearchQuery) -> Vec<Asset> {
        self.list_active_assets()
            .await
            .into_iter()
            .filter(|e| {
                let name_ok = query
                    .name
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|n| e.name.as_str() == n)
                    .unwrap_or(true);
                let fuzzy_ok = query
                    .fuzzyname
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|n| e.name.as_str().to_lowercase().contains(&n.to_lowercase()))
                    .unwrap_or(true);
                name_ok && fuzzy_ok
            })
            .collect()
    }

    async fn list_child_assets(&self, parent_id: AssetId) -> Vec<Asset> {
        self.list_active_assets()
            .await
            .into_iter()
            .filter(|e| e.parent == Some(parent_id) && e.id != parent_id)
            .collect()
    }
}

pub struct StubAssetCommand {
    next_id: AtomicI64,
    assets: Mutex<HashMap<i64, Asset>>,
}

impl Default for StubAssetCommand {
    fn default() -> Self {
        Self {
            next_id: AtomicI64::new(1),
            assets: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl AssetCommandPort for StubAssetCommand {
    async fn create_asset(&self, name: AssetName, parent: Option<AssetId>) -> Asset {
        let id_raw = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = AssetId::from(id_raw);
        let entity = Asset::new(id, name, parent, AssetTimestamp::now(), None);

        let mut assets = self.assets.lock().unwrap();
        assets.insert(id_raw, entity.clone());

        entity
    }
    async fn delete_asset(&self, id: AssetId) -> Result<(), String> {
        let mut assets = self.assets.lock().map_err(|_| "Poisoned lock")?;
        let id_raw = i64::from(id);

        if let Some(entity) = assets.get_mut(&id_raw) {
            if entity.is_deleted() {
                return Err(format!("Asset with ID {} already deleted", id_raw));
            }

            entity.mark_as_deleted();

            Ok(())
        } else {
            Err(format!(
                "Entity with ID {} not found in stub memory",
                id_raw
            ))
        }
    }
    async fn update_asset(&self, id: AssetId, changes: PartialAsset) -> Result<(), String> {
        let mut storage = self.assets.lock().unwrap();
        let id_raw = i64::from(id);

        if let Some(entity) = storage.get_mut(&id_raw) {
            if let Some(new_name) = changes.name {
                entity.name = new_name;
            }
            if let Some(new_parent) = changes.parent {
                entity.parent = new_parent;
            }
            Ok(())
        } else {
            Err("Entity not found".into())
        }
    }
}
