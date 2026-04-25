#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct EntityId(i64);

impl From<i64> for EntityId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<EntityId> for i64 {
    fn from(id: EntityId) -> Self {
        id.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityName(String);

impl From<&str> for EntityName {
    fn from(s: &str) -> Self {
        EntityName(s.to_string())
    }
}

impl From<String> for EntityName {
    fn from(s: String) -> Self {
        EntityName(s)
    }
}

impl std::ops::Deref for EntityName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EntityName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entity {
    pub id: EntityId,
    pub name: EntityName,
    pub parent: Option<EntityId>,
}

impl Entity {
    pub fn new(id: EntityId, name: EntityName, parent: Option<EntityId>) -> Self {
        Self { id, name, parent }
    }
}

#[cfg(test)]
mod item_tests {
    use crate::item::{Entity, EntityId, EntityName};

    #[test]
    fn change_name() {
        let mut item = Entity::new(EntityId::from(1), EntityName::from("Fork"), None);
        item.name = EntityName::from("Spoon");
        assert_eq!(item.name, EntityName::from("Spoon"))
    }
}
