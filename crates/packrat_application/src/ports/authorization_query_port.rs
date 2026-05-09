use async_trait::async_trait;
use packrat_domain::tenant::TenantId;
use packrat_domain::user::UserId;
use packrat_domain::PermissionSlug;

#[async_trait]
pub trait AuthorizationQueryPort: Send + Sync {
    /// Returns whether `user_id` has `slug` in `tenant_id` via roles and/or direct `user_permissions`.
    async fn user_has_permission(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        slug: PermissionSlug,
    ) -> Result<bool, String>;
}
