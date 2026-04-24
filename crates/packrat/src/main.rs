//! Composition root: the only place `infrastructure` and the `packrat_application` layer are wired.

use packrat_domain::Item;
use packrat_infrastructure::StubItemQuery;
use packrat_application::get_example_item;

fn main() {
    let item_query = StubItemQuery;
    if let Some(item) = get_example_item(&item_query) {
        println!("#{:?}: {:?}", item.id, item.name);
    }
    let local = Item::new(1, String::from("Test"));
    println!("#{:?}: {:?}", local.id, local.name);
}
