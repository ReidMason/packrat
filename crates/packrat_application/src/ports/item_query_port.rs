use packrat_domain::item::{Entity, EntityId};

pub trait ItemQueryPort: Send + Sync {
    fn get_item_by_id(&self, id: EntityId) -> Option<Entity>;
}
