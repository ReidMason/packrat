#[derive(Debug)]
pub struct ItemId(u64);

impl ItemId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, PartialEq)]
pub struct ItemName(String);

impl ItemName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

pub struct Item {
    id: ItemId,
    name: ItemName,
}

impl Item {
    pub fn new(id: ItemId, name: ItemName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &ItemId {
        &self.id
    }

    pub fn name(&self) -> &ItemName {
        &self.name
    }

    pub fn id_mut(&mut self) -> &mut ItemId {
        &mut self.id
    }

    pub fn name_mut(&mut self) -> &mut ItemName {
        &mut self.name
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
        *item.name_mut() = ItemName::new("Spoon");
        assert_eq!(item.name(), &ItemName::new("Spoon"))
    }
}
