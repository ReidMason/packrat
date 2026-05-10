use packrat_domain::tag::Tag;
use packrat_domain::tenant::TenantId;

use crate::ports::TagQueryPort;

pub async fn execute(
    port: &impl TagQueryPort,
    tenant_id: TenantId,
    prefix: Option<&str>,
) -> Vec<Tag> {
    port.list_tags(tenant_id, prefix).await
}
