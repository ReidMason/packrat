use packrat_domain::item::{Item, ItemId};

use crate::ports::ItemQueryPort;

pub fn execute(port: &impl ItemQueryPort, id: ItemId) -> Option<Item> {
    port.get_item_by_id(id)
}

#[cfg(test)]
mod tests {
    use packrat_domain::inventory::InventoryId;
    use packrat_domain::item::ItemName;
    use packrat_domain::location::LocationId;

    use super::*;

    struct MockItemQuery;

    fn stub_item(id: ItemId) -> Item {
        Item::new(
            id,
            ItemName::from("from infrastructure stub"),
            InventoryId::Location(LocationId::new(1)),
        )
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
