//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

mod postgres;
mod postgres_user;
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
pub use postgres_user::PostgresUserCommand;
pub use readiness::PostgresReadiness;

use packrat_application::{AssetCommandPort, AssetQueryPort, AssetSearchQuery};
use packrat_domain::{
    entity::{Entity, EntityId, EntityName, EntityTimestamp},
    models::partial_entity::PartialEntity,
};

fn stub_entity(id: EntityId) -> Entity {
    Entity::new(
        id,
        EntityName::from("from infrastructure stub"),
        Some(EntityId::from(1)),
        EntityTimestamp::now(),
        None,
    )
}

/// Placeholder “database” for wiring demos and tests.
pub struct StubAssetQuery;

#[async_trait]
impl AssetQueryPort for StubAssetQuery {
    async fn get_asset_by_id(&self, id: EntityId) -> Option<Entity> {
        if id == EntityId::from(1) {
            Some(stub_entity(id))
        } else {
            None
        }
    }

    async fn list_active_assets(&self) -> Vec<Entity> {
        vec![stub_entity(EntityId::from(1))]
    }

    async fn search_assets(&self, query: &AssetSearchQuery) -> Vec<Entity> {
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

    async fn list_child_assets(&self, parent_id: EntityId) -> Vec<Entity> {
        self.list_active_assets()
            .await
            .into_iter()
            .filter(|e| e.parent == Some(parent_id) && e.id != parent_id)
            .collect()
    }
}

pub struct StubAssetCommand {
    next_id: AtomicI64,
    assets: Mutex<HashMap<i64, Entity>>,
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
    async fn create_asset(&self, name: EntityName, parent: Option<EntityId>) -> Entity {
        let id_raw = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = EntityId::from(id_raw);
        let entity = Entity::new(id, name, parent, EntityTimestamp::now(), None);

        let mut assets = self.assets.lock().unwrap();
        assets.insert(id_raw, entity.clone());

        entity
    }
    async fn delete_asset(&self, id: EntityId) -> Result<(), String> {
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
    async fn update_asset(&self, id: EntityId, changes: PartialEntity) -> Result<(), String> {
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
