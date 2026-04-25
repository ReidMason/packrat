//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

mod postgres;

use async_trait::async_trait;
use std::sync::atomic::{AtomicI64, Ordering};

pub use postgres::{PostgresItemCommand, connect_pool, run_migrations};

use packrat_application::{ItemCommandPort, ItemQueryPort};
use packrat_domain::item::{Entity, EntityId, EntityName};

fn stub_item(id: EntityId) -> Entity {
    Entity::new(
        id,
        EntityName::from("from infrastructure stub"),
        Some(EntityId::from(1)),
    )
}

/// Placeholder “database” for wiring demos and tests.
pub struct StubItemQuery;

impl ItemQueryPort for StubItemQuery {
    fn get_item_by_id(&self, id: EntityId) -> Option<Entity> {
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
        Entity::new(id, name, parent)
    }
}
