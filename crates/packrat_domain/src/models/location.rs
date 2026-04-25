use super::Id;

#[derive(Debug, PartialEq, Eq)]
pub struct LocationName(String);

impl From<&str> for LocationName {
    fn from(s: &str) -> Self {
        LocationName(s.to_string())
    }
}

impl From<String> for LocationName {
    fn from(s: String) -> Self {
        LocationName(s)
    }
}

impl std::ops::Deref for LocationName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LocationName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Location {
    pub id: Id,
    pub name: LocationName,
}

impl Location {
    pub fn new(id: Id, name: LocationName) -> Self {
        Self { id, name }
    }
}
