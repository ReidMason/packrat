//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

mod postgres;
mod readiness;

use async_trait::async_trait;
use std::{
    collections::HashMap,
    sync::{
        Mutex,
        atomic::{AtomicI64, Ordering},
    },
};

pub use postgres::{PostgresItemCommand, PostgresItemQuery, connect_pool, ping_database, run_migrations};
pub use readiness::PostgresReadiness;

use packrat_application::{ItemCommandPort, ItemQueryPort};
use packrat_domain::{
    entity::{Entity, EntityId, EntityName, EntityTimestamp},
    models::partial_entity::PartialEntity,
};

fn stub_item(id: EntityId) -> Entity {
    Entity::new(
        id,
        EntityName::from("from infrastructure stub"),
        Some(EntityId::from(1)),
        EntityTimestamp::now(),
        None,
    )
}

/// Placeholder “database” for wiring demos and tests.
pub struct StubItemQuery;

#[async_trait]
impl ItemQueryPort for StubItemQuery {
    async fn get_item_by_id(&self, id: EntityId) -> Option<Entity> {
        if id == EntityId::from(1) {
            Some(stub_item(id))
        } else {
            None
        }
    }
}

pub struct StubItemCommand {
    next_id: AtomicI64,
    items: Mutex<HashMap<i64, Entity>>,
}

impl Default for StubItemCommand {
    fn default() -> Self {
        Self {
            next_id: AtomicI64::new(1),
            items: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl ItemCommandPort for StubItemCommand {
    async fn create_item(&self, name: EntityName, parent: Option<EntityId>) -> Entity {
        let id_raw = self.next_id.fetch_add(1, Ordering::Relaxed);
        let id = EntityId::from(id_raw);
        let entity = Entity::new(id, name, parent, EntityTimestamp::now(), None);

        let mut items = self.items.lock().unwrap();
        items.insert(id_raw, entity.clone());

        entity
    }
    async fn delete_entity(&self, id: EntityId) -> Result<(), String> {
        let mut items = self.items.lock().map_err(|_| "Poisoned lock")?;
        let id_raw = i64::from(id);

        if let Some(entity) = items.get_mut(&id_raw) {
            if entity.is_deleted() {
                return Err(format!("Item with ID {} already deleted", id_raw));
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
    async fn update_entity(&self, id: EntityId, changes: PartialEntity) -> Result<(), String> {
        let mut storage = self.items.lock().unwrap();
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
