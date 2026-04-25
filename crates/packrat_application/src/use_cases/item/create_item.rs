use packrat_domain::item::{Item, ItemName};

use crate::ports::ItemCommandPort;

pub fn execute(port: &impl ItemCommandPort, name: ItemName) -> Item {
    port.create_item(name)
}

#[cfg(test)]
mod tests {
    use packrat_domain::item::ItemId;

    use super::*;

    struct MockItemCommand;

    impl ItemCommandPort for MockItemCommand {
        fn create_item(&self, name: ItemName) -> Item {
            Item::new(ItemId::new(99), name)
        }
    }

    #[test]
    fn execute_creates_item_via_port() {
        let port = MockItemCommand;
        let item = execute(&port, ItemName::from("alpha"));
        assert_eq!(item.id, ItemId::new(99));
        assert_eq!(item.name, ItemName::from("alpha"));
    }
}
