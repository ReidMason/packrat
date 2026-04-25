#[derive(Debug, PartialEq, Eq)]
pub struct ItemId(u64);

impl ItemId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ItemName(String);

impl ItemName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub id: ItemId,
    pub name: ItemName,
}

impl Item {
    pub fn new(id: ItemId, name: ItemName) -> Self {
        Self { id, name }
    }
}

impl std::ops::Deref for ItemName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ItemName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod item_tests {
    use crate::{Item, ItemId, ItemName};

    #[test]
    fn change_name() {
        let mut item = Item::new(ItemId::new(1), ItemName::new("Fork"));
        item.name = ItemName::new("Spoon");
        assert_eq!(item.name, ItemName::new("Spoon"))
    }
}
