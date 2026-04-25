//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

use std::sync::atomic::{AtomicU64, Ordering};

use packrat_application::{ItemCommandPort, ItemQueryPort};
use packrat_domain::{Item, ItemId, ItemName};

fn stub_item(id: ItemId) -> Item {
    Item::new(id, ItemName::new("from infrastructure stub"))
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

impl ItemCommandPort for StubItemCommand {
    fn create_item(&self, name: ItemName) -> Item {
        let id = ItemId::new(self.next_id.fetch_add(1, Ordering::Relaxed));
        Item::new(id, name)
    }
}
