use packrat_domain::item::{Item, ItemName};

use crate::ports::ItemCommandPort;

pub async fn execute(port: &impl ItemCommandPort, name: ItemName) -> Item {
    port.create_item(name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::item::ItemId;

    struct MockItemCommand;

    #[async_trait]
    impl ItemCommandPort for MockItemCommand {
        async fn create_item(&self, name: ItemName) -> Item {
            Item::new(ItemId::new(99), name)
        }
    }

    #[tokio::test]
    async fn execute_creates_item_via_port() {
        let port = MockItemCommand;
        let item = execute(&port, ItemName::from("alpha")).await;
        assert_eq!(item.id, ItemId::new(99));
        assert_eq!(item.name, ItemName::from("alpha"));
    }
}
