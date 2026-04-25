use super::{Id, Parent};

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
    pub id: Id,
    pub name: ItemName,
    pub parent: Option<Parent>,
}

impl Item {
    pub fn new(id: Id, name: ItemName, parent: Option<Parent>) -> Self {
        Self { id, name, parent }
    }
}

#[cfg(test)]
mod item_tests {
    use crate::item::{Item, Id, ItemName};

    #[test]
    fn change_name() {
        let mut item = Item::new(Id::new(1), ItemName::from("Fork"), None);
        item.name = ItemName::from("Spoon");
        assert_eq!(item.name, ItemName::from("Spoon"))
    }
}
