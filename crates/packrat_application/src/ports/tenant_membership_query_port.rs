use async_trait::async_trait;
use packrat_domain::tenant::Tenant;
use packrat_domain::user::UserId;

#[async_trait]
pub trait TenantMembershipQueryPort: Send + Sync {
    async fn list_tenants_for_user(&self, user_id: UserId) -> Result<Vec<Tenant>, String>;
}
