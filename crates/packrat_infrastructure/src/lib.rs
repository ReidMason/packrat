//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

use std::sync::atomic::{AtomicU64, Ordering};

use packrat_application::{ItemCommandPort, ItemQueryPort};
use packrat_domain::{
    item::{Item, ItemName},
    models::Id,
};

fn stub_item(id: Id) -> Item {
    Item::new(id, ItemName::from("from infrastructure stub"), None)
}

/// Placeholder “database” for wiring demos and tests.
pub struct StubItemQuery;

impl ItemQueryPort for StubItemQuery {
    fn get_item_by_id(&self, id: Id) -> Option<Item> {
        if id == Id::new(1) {
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
        let id = Id::new(self.next_id.fetch_add(1, Ordering::Relaxed));
        Item::new(id, name, None)
    }
}
