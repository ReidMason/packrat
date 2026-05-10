#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TagName(String);

impl TagName {
    /// Trimmed display name; fails if empty after trim.
    pub fn parse(input: &str) -> Result<Self, &'static str> {
        let t = input.trim();
        if t.is_empty() {
            Err("tag name must not be empty")
        } else {
            Ok(TagName(t.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Case-insensitive identity: `lower(trim(original))`.
    pub fn normalized(&self) -> String {
        self.0.to_lowercase()
    }
}

impl From<&str> for TagName {
    fn from(s: &str) -> Self {
        TagName(s.to_string())
    }
}

impl From<String> for TagName {
    fn from(s: String) -> Self {
        TagName(s)
    }
}

impl From<TagName> for String {
    fn from(name: TagName) -> Self {
        name.0
    }
}
