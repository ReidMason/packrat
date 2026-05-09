//! Stable permission slugs aligned with `permissions.slug` in Postgres migrations.

/// Permission checked for tenant-scoped authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermissionSlug {
    AssetsRead,
    AssetsWrite,
    AssetsDelete,
}

impl PermissionSlug {
    /// Value stored in `permissions.slug`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssetsRead => "assets.read",
            Self::AssetsWrite => "assets.write",
            Self::AssetsDelete => "assets.delete",
        }
    }
}
