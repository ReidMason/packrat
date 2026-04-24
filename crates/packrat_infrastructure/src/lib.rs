//! Adapters: persistence, APIs, OS. Implements ports from `packrat_application`.

use packrat_domain::Item;
use packrat_application::ItemQueryPort;

/// Placeholder “database” for wiring demos and tests.
pub struct StubItemQuery;

impl ItemQueryPort for StubItemQuery {
    fn fetch_example(&self) -> Option<Item> {
        Some(Item::new(1, "from infrastructure stub".to_string()))
    }
}
