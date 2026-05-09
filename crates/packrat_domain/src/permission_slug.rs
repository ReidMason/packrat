#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionSlug {
    AssetsRead,
    AssetsWrite,
    AssetsDelete,
}

impl PermissionSlug {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssetsRead => "assets.read",
            Self::AssetsWrite => "assets.write",
            Self::AssetsDelete => "assets.delete",
        }
    }
}
