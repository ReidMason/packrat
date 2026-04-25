use packrat_domain::{item::Item, models::Id};

use crate::ports::ItemQueryPort;

pub fn execute(port: &impl ItemQueryPort, id: Id) -> Option<Item> {
    port.get_item_by_id(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use packrat_domain::{item::ItemName, models::Id};

    struct MockItemQuery;

    fn stub_item(id: Id) -> Item {
        Item::new(id, ItemName::from("from infrastructure stub"), None)
    }

    impl ItemQueryPort for MockItemQuery {
        fn get_item_by_id(&self, id: Id) -> Option<Item> {
            if id == Id::new(1) {
                Some(stub_item(id))
            } else {
                None
            }
        }
    }

    #[test]
    fn execute_test() {
        let port = MockItemQuery;
        assert_eq!(execute(&port, Id::new(1)), Some(stub_item(Id::new(1))));
    }
}
