use packrat_domain::item::{Item, ItemName};

use crate::ports::{ItemCommandPort, ItemPlacement};

pub async fn execute(
    port: &impl ItemCommandPort,
    name: ItemName,
    placement: ItemPlacement,
) -> Item {
    port.create_item(name, placement).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use packrat_domain::item::ItemId;
    use packrat_domain::location::LocationId;

    struct MockItemCommand;

    #[async_trait]
    impl ItemCommandPort for MockItemCommand {
        async fn create_item(&self, name: ItemName, _placement: ItemPlacement) -> Item {
            Item::new(ItemId::new(99), name, None)
        }
    }

    #[tokio::test]
    async fn execute_creates_item_via_port() {
        let port = MockItemCommand;
        let item = execute(
            &port,
            ItemName::from("alpha"),
            ItemPlacement::InLocation(LocationId::new(1)),
        )
        .await;
        assert_eq!(item.id, ItemId::new(99));
        assert_eq!(item.name, ItemName::from("alpha"));
    }
}
