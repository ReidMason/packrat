#[derive(Debug, PartialEq, Eq)]
pub struct ItemName(String);

impl ItemName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ItemId(u64);

impl ItemId {
    pub fn new(id: u64) -> Self {
        Self(id)
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
