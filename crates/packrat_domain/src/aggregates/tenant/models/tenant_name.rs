#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TenantName(String);

impl TenantName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for TenantName {
    fn from(s: &str) -> Self {
        TenantName(s.to_string())
    }
}

impl From<String> for TenantName {
    fn from(s: String) -> Self {
        TenantName(s)
    }
}

impl From<TenantName> for String {
    fn from(name: TenantName) -> Self {
        name.0
    }
}

