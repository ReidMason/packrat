//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

use packrat_application::ItemQueryPort;
use packrat_domain::{Item, ItemId, ItemName};

/// Placeholder “database” for wiring demos and tests.
pub struct StubItemQuery;

impl ItemQueryPort for StubItemQuery {
    fn fetch_example(&self) -> Option<Item> {
        Some(Item::new(
            ItemId::new(1),
            ItemName::new("from infrastructure stub"),
        ))
    }
}
