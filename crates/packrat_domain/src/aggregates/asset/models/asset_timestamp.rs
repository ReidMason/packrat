#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct AssetTimestamp(chrono::DateTime<chrono::Utc>);

impl AssetTimestamp {
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }

    pub fn static_for_tests() -> Self {
        chrono::DateTime::from_timestamp(1735689600, 0)
            .unwrap()
            .into()
    }
}

impl std::fmt::Display for AssetTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for AssetTimestamp {
    fn from(dt: chrono::DateTime<chrono::Utc>) -> Self {
        Self(dt)
    }
}

impl From<AssetTimestamp> for chrono::DateTime<chrono::Utc> {
    fn from(ts: AssetTimestamp) -> Self {
        ts.0
    }
}
