use async_trait::async_trait;
use packrat_domain::tag::Tag;
use packrat_domain::tenant::TenantId;

#[async_trait]
pub trait TagQueryPort: Send + Sync {
    /// List tags for the tenant. When `prefix` is `None` or empty after trim, returns all tags
    /// ordered by name. Otherwise returns tags whose normalized name starts with the normalized prefix.
    async fn list_tags(&self, tenant_id: TenantId, prefix: Option<&str>) -> Vec<Tag>;
}
