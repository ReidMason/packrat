use bucket::BucketId;
use location::LocationId;

pub mod bucket;
pub mod item;
pub mod location;

#[derive(Debug, PartialEq, Eq)]
pub enum Parent {
    Bucket(BucketId),
    Location(LocationId),
}
