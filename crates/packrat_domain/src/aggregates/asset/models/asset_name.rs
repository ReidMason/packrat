#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssetName(String);

impl AssetName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for AssetName {
    fn from(s: &str) -> Self {
        AssetName(s.to_string())
    }
}

impl From<String> for AssetName {
    fn from(s: String) -> Self {
        AssetName(s)
    }
}

impl From<AssetName> for String {
    fn from(name: AssetName) -> Self {
        name.0
    }
}

impl std::ops::Deref for AssetName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AssetName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
