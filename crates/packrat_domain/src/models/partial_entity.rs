use crate::entity::{EntityId, EntityName};

#[derive(Debug, PartialEq, Eq, Default)]
pub struct PartialEntity {
    pub name: Option<EntityName>,
    pub parent: Option<Option<EntityId>>,
}

impl PartialEntity {
    pub fn new(name: Option<EntityName>, parent: Option<Option<EntityId>>) -> Self {
        Self { name, parent }
    }
}
