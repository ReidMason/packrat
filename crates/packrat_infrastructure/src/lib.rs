//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

mod postgres;

use async_trait::async_trait;
use std::sync::atomic::{AtomicU64, Ordering};

pub use postgres::{connect_pool, run_migrations, PostgresItemCommand};

use packrat_application::{ItemCommandPort, ItemPlacement, ItemQueryPort};
use packrat_domain::item::{Item, ItemId, ItemName};

fn stub_item(id: ItemId) -> Item {
    Item::new(id, ItemName::from("from infrastructure stub"))
}

/// Placeholder “database” for wiring demos and tests.
pub struct StubItemQuery;

impl ItemQueryPort for StubItemQuery {
    fn get_item_by_id(&self, id: ItemId) -> Option<Item> {
        if id == ItemId::new(1) {
            Some(stub_item(id))
        } else {
            None
        }
    }
}

pub struct StubItemCommand {
    next_id: AtomicU64,
}

impl Default for StubItemCommand {
    fn default() -> Self {
        Self {
            next_id: AtomicU64::new(1),
        }
    }
}

#[async_trait]
impl ItemCommandPort for StubItemCommand {
    async fn create_item(&self, name: ItemName, _placement: ItemPlacement) -> Item {
        let id = ItemId::new(self.next_id.fetch_add(1, Ordering::Relaxed));
        Item::new(id, name)
    }
}
