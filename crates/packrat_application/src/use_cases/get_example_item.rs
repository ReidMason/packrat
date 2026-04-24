use packrat_domain::Item;

use crate::ports::ItemQueryPort;

pub fn get_example_item(port: &impl ItemQueryPort) -> Option<Item> {
    port.fetch_example()
}

#[cfg(test)]
mod tests {
    use super::*;
    use packrat_domain::{ItemId, ItemName};

    struct MockItemQuery;

    fn example_stub_item() -> Item {
        Item::new(
            ItemId::new(1),
            ItemName::new("from infrastructure stub"),
        )
    }

    impl ItemQueryPort for MockItemQuery {
        fn fetch_example(&self) -> Option<Item> {
            Some(example_stub_item())
        }
    }

    #[test]
    fn get_example_item_test() {
        let port = MockItemQuery;
        assert_eq!(get_example_item(&port), Some(example_stub_item()));
    }
}
