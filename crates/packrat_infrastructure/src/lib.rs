//! Adapters: persistence, APIs, OS. This crate may depend on `packrat_domain` only
//! (plus driver crates). `packrat_domain` must not depend on this crate.

use packrat_domain::Item;

/// Example secondary adapter. Real code would use a database client from here.
pub fn fetch_example_item() -> Option<Item> {
    Some(Item::new(1, "from infrastructure stub".to_string()))
}
