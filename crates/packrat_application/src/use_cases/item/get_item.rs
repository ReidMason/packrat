use packrat_domain::item::{Entity, EntityId};

use crate::ports::ItemQueryPort;

pub fn execute(port: &impl ItemQueryPort, id: EntityId) -> Option<Entity> {
    port.get_item_by_id(id)
}

#[cfg(test)]
mod tests {
    use packrat_domain::item::EntityName;

    use super::*;

    struct MockItemQuery;

    fn stub_item(id: EntityId) -> Entity {
        Entity::new(
            id,
            EntityName::from("from infrastructure stub"),
            Some(EntityId::from(1)),
        )
    }

    impl ItemQueryPort for MockItemQuery {
        fn get_item_by_id(&self, id: EntityId) -> Option<Entity> {
            if id == EntityId::from(1) {
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
            execute(&port, EntityId::from(1)),
            Some(stub_item(EntityId::from(1)))
        );
    }
}
