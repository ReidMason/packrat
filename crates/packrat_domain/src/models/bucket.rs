use crate::{
    inventory::Inventory,
    stock::{Stock, StockId},
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BucketId(u64);

impl BucketId {
    pub fn new(id: u64) -> Self {
        Self(id)
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
    fn find_item(&self, id: StockId) -> Option<&dyn Stock> {
        for item in &self.stock {
            if item.id() == id {
                return Some(item.as_ref());
            }

            if let Some(inventory) = item.as_inventory() {
                if let Some(found) = inventory.find_item(id) {
                    return Some(found);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod bucket_tests {
    use super::*;
    use crate::item::{Item, ItemId, ItemName};

    fn setup_test_buckets() -> Bucket {
        let item = Item::new(ItemId::new(100), ItemName::from("Spoon"));

        let sub_bucket = Bucket {
            id: BucketId::new(2),
            name: BucketName::from("Little Box"),
            stock: vec![Box::new(item)],
        };

        Bucket {
            id: BucketId::new(1),
            name: BucketName::from("Big Box"),
            stock: vec![Box::new(sub_bucket)],
        }
    }

    #[test]
    fn test_find_item_in_sub_bucket() {
        let bucket = setup_test_buckets();
        let item_id = StockId::Item(ItemId::new(100));

        let found = bucket.find_item(item_id);

        assert!(
            found.is_some(),
            "Should find the item nested inside the sub-bucket"
        );
        assert_eq!(found.unwrap().id(), item_id);
    }

    #[test]
    fn test_find_direct_child_bucket() {
        let bucket = setup_test_buckets();
        let sub_bucket_id = StockId::Bucket(BucketId::new(2));

        let found = bucket.find_item(sub_bucket_id);

        assert!(
            found.is_some(),
            "Should be able to find a bucket that is a direct child"
        );
        assert_eq!(found.unwrap().id(), sub_bucket_id);
    }

    #[test]
    fn test_find_item_not_found() {
        let root = setup_test_buckets();
        let missing_id = StockId::Item(ItemId::new(999));

        let found = root.find_item(missing_id);

        assert!(
            found.is_none(),
            "Should return None for IDs that do not exist"
        );
    }
}
