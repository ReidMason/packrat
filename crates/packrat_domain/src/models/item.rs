use super::Parent;

// TODO: Id's should probably be a UUID
#[derive(Debug, PartialEq, Eq)]
pub struct ItemId(u64);

impl ItemId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ItemName(String);

impl From<&str> for ItemName {
    fn from(s: &str) -> Self {
        ItemName(s.to_string())
    }
}

impl From<String> for ItemName {
    fn from(s: String) -> Self {
        ItemName(s)
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

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub id: ItemId,
    pub name: ItemName,
    pub parent: Option<Parent>,
}

impl Item {
    pub fn new(id: ItemId, name: ItemName, parent: Option<Parent>) -> Self {
        Self { id, name, parent }
    }
}

#[cfg(test)]
mod item_tests {
    use crate::item::{Item, ItemId, ItemName};

    #[test]
    fn change_name() {
        let mut item = Item::new(ItemId::new(1), ItemName::from("Fork"), None);
        item.name = ItemName::from("Spoon");
        assert_eq!(item.name, ItemName::from("Spoon"))
    }
}
