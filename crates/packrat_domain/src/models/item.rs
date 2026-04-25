use crate::{inventory::InventoryId, stock::{Stock, StockId}};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ItemId(i64);

impl ItemId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
}

impl From<ItemId> for i64 {
    fn from(id: ItemId) -> Self {
        id.0
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
    pub parent: InventoryId,
}

impl Item {
    pub fn new(id: ItemId, name: ItemName, parent: InventoryId) -> Self {
        Self { id, name, parent }
    }
}

impl Stock for Item {
    fn id(&self) -> StockId {
        StockId::Item(self.id)
    }
}

#[cfg(test)]
mod item_tests {
    use crate::inventory::InventoryId;
    use crate::item::{Item, ItemId, ItemName};
    use crate::location::LocationId;

    #[test]
    fn change_name() {
        let mut item = Item::new(
            ItemId::new(1),
            ItemName::from("Fork"),
            InventoryId::Location(LocationId::new(1)),
        );
        item.name = ItemName::from("Spoon");
        assert_eq!(item.name, ItemName::from("Spoon"))
    }
}
