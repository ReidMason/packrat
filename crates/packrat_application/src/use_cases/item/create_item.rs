use packrat_domain::{Item, ItemName};

use crate::ports::ItemCommandPort;

pub fn execute(port: &impl ItemCommandPort, name: ItemName) -> Item {
    port.create_item(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use packrat_domain::ItemId;

    struct MockItemCommand;

    impl ItemCommandPort for MockItemCommand {
        fn create_item(&self, name: ItemName) -> Item {
            Item::new(ItemId::new(99), name)
        }
    }

    #[test]
    fn execute_creates_item_via_port() {
        let port = MockItemCommand;
        let item = execute(&port, ItemName::new("alpha"));
        assert_eq!(item.id, ItemId::new(99));
        assert_eq!(item.name, ItemName::new("alpha"));
    }
}
