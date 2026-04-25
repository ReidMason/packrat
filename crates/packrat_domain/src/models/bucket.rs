use crate::{
    inventory::{Inventory, InventoryId},
    stock::{Stock, StockId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BucketId(u64);

impl BucketId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BucketName(String);

impl From<&str> for BucketName {
    fn from(s: &str) -> Self {
        BucketName(s.to_string())
    }
}

impl From<String> for BucketName {
    fn from(s: String) -> Self {
        BucketName(s)
    }
}

impl std::ops::Deref for BucketName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for BucketName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Bucket {
    pub id: BucketId,
    pub name: BucketName,
    pub parent: InventoryId,
    pub stock: Vec<Box<dyn Stock>>,
}

impl Stock for Bucket {
    fn id(&self) -> StockId {
        StockId::Bucket(self.id)
    }

    fn as_inventory(&self) -> Option<&dyn Inventory> {
        Some(self)
    }
}

impl Inventory for Bucket {
    fn find_item(&self, _id: StockId) -> Option<&dyn Stock> {
        todo!()
    }
}
