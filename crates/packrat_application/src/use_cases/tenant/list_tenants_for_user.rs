use packrat_domain::tenant::Tenant;
use packrat_domain::user::UserId;

use crate::ports::TenantMembershipQueryPort;

pub async fn execute(
    port: &impl TenantMembershipQueryPort,
    user_id: UserId,
) -> Result<Vec<Tenant>, String> {
    port.list_tenants_for_user(user_id).await
}
