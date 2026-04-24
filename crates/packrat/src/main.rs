//! `packrat` (binary): composition root. Safe to use both domain and infrastructure.
//! The `packrat_domain` crate is forbidden from depending on this or `packrat_infrastructure`.

use packrat_domain::Item;
use packrat_infrastructure::fetch_example_item;

fn main() {
    if let Some(item) = fetch_example_item() {
        println!("#{:?}: {:?}", item.id, item.name);
    }
    let local = Item::new(1, String::from("Test"));
    println!("#{:?}: {:?}", local.id, local.name);
}
