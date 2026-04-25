use packrat_domain::{Item, ItemId};

use crate::ports::ItemQueryPort;

pub fn execute(port: &impl ItemQueryPort, id: ItemId) -> Option<Item> {
    port.get_item_by_id(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use packrat_domain::{ItemId, ItemName};

    struct MockItemQuery;

    fn stub_item(id: ItemId) -> Item {
        Item::new(id, ItemName::new("from infrastructure stub"))
    }

    impl ItemQueryPort for MockItemQuery {
        fn get_item_by_id(&self, id: ItemId) -> Option<Item> {
            if id == ItemId::new(1) {
                Some(stub_item(id))
            } else {
                None
            }
        }
    }

    #[test]
    fn execute_test() {
        let port = MockItemQuery;
        assert_eq!(
            execute(&port, ItemId::new(1)),
            Some(stub_item(ItemId::new(1)))
        );
    }
}
