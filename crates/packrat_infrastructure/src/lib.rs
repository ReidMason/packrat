//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

mod postgres;
mod readiness;

use async_trait::async_trait;
use std::sync::atomic::{AtomicI64, Ordering};

pub use postgres::{PostgresItemCommand, PostgresItemQuery, connect_pool, ping_database, run_migrations};
pub use readiness::PostgresReadiness;

use packrat_application::{ItemCommandPort, ItemQueryPort};
use packrat_domain::entity::{Entity, EntityId, EntityName, EntityTimestamp};

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
}

impl Default for StubItemCommand {
    fn default() -> Self {
        Self {
            next_id: AtomicI64::new(1),
        }
    }
}

#[async_trait]
impl ItemCommandPort for StubItemCommand {
    async fn create_item(&self, name: EntityName, parent: Option<EntityId>) -> Entity {
        let id = EntityId::from(self.next_id.fetch_add(1, Ordering::Relaxed));
        let created = EntityTimestamp::now();
        let deleted = None;
        Entity::new(id, name, parent, created, deleted)
    }
    async fn delete_entity(&self, id: EntityId) -> Result<(), String> {
        let current_max = self.next_id.load(Ordering::Relaxed);
        let target_id = i64::from(id);

        if target_id > 0 && target_id < current_max {
            Ok(())
        } else {
            Err("Entity not found in stub memory".to_string())
        }
    }
}
