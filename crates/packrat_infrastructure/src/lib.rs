//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

use packrat_application::ItemQueryPort;
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
