use super::{Id, Parent};

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
    pub id: Id,
    pub name: BucketName,
    pub parent: Option<Parent>,
}
