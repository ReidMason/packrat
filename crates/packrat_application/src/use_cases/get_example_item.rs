use packrat_domain::Item;

use crate::ports::ItemQueryPort;

pub fn get_example_item(port: &impl ItemQueryPort) -> Option<Item> {
    port.fetch_example()
}
