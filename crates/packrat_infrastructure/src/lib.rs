//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

mod postgres;

use async_trait::async_trait;
use std::sync::atomic::{AtomicI64, Ordering};

pub use postgres::{connect_pool, run_migrations, PostgresItemCommand};

use packrat_application::{ItemCommandPort, ItemPlacement, ItemQueryPort};
use packrat_domain::inventory::InventoryId;
use packrat_domain::item::{Item, ItemId, ItemName};
use packrat_domain::location::LocationId;

fn stub_item(id: ItemId) -> Item {
    Item::new(
        id,
        ItemName::from("from infrastructure stub"),
        InventoryId::Location(LocationId::from(1)),
    )
}

/// Placeholder “database” for wiring demos and tests.
pub struct StubItemQuery;

impl ItemQueryPort for StubItemQuery {
    fn get_item_by_id(&self, id: ItemId) -> Option<Item> {
        if id == ItemId::from(1) {
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
    async fn create_item(&self, name: ItemName, placement: ItemPlacement) -> Item {
        let id = ItemId::from(self.next_id.fetch_add(1, Ordering::Relaxed));
        let parent = match placement {
            ItemPlacement::InLocation(loc) => InventoryId::Location(loc),
            ItemPlacement::InBucket(bid) => InventoryId::Bucket(bid),
        };
        Item::new(id, name, parent)
    }
}
