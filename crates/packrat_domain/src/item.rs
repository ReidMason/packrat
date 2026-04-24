#[derive(Debug)]
pub struct ItemName(#[allow(dead_code)] String);

#[derive(Debug)]
pub struct ItemId(#[allow(dead_code)] u64);

pub struct Item {
    pub id: ItemId,
    pub name: ItemName,
}

impl Item {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id: ItemId(id),
            name: ItemName(name),
        }
    }
}
