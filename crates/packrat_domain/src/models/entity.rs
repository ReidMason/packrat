#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct EntityId(i64);

impl From<i64> for EntityId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<EntityId> for i64 {
    fn from(id: EntityId) -> Self {
        id.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityName(String);

impl From<&str> for EntityName {
    fn from(s: &str) -> Self {
        EntityName(s.to_string())
    }
}

impl From<String> for EntityName {
    fn from(s: String) -> Self {
        EntityName(s)
    }
}

impl std::ops::Deref for EntityName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for EntityName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct EntityTimestamp(chrono::DateTime<chrono::Utc>);

impl EntityTimestamp {
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }
}

impl std::fmt::Display for EntityTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for EntityTimestamp {
    fn from(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self(dt)
    }
}

impl From<EntityTimestamp> for chrono::DateTime<chrono::Utc> {
    fn from(ts: EntityTimestamp) -> Self {
        ts.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Entity {
    pub id: EntityId,
    pub name: EntityName,
    pub parent: Option<EntityId>,
    pub created: EntityTimestamp,
    pub deleted: Option<EntityTimestamp>,
}

impl Entity {
    pub fn new(
        id: EntityId,
        name: EntityName,
        parent: Option<EntityId>,
        created: EntityTimestamp,
        deleted: Option<EntityTimestamp>,
    ) -> Self {
        Self {
            id,
            name,
            parent,
            created,
            deleted,
        }
    }
}

#[cfg(test)]
mod item_tests {}
