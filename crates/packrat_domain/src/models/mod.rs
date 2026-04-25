pub mod bucket;
pub mod item;
pub mod location;

// TODO: This should probably be a UUID
#[derive(Debug, PartialEq, Eq)]
pub struct Id(u64);

impl Id {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Parent {
    Bucket(Id),
    Location(Id),
}
