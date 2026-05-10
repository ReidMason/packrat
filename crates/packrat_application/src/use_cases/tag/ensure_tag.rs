use packrat_domain::tag::{Tag, TagName};
use packrat_domain::tenant::TenantId;

use crate::ports::TagCommandPort;

pub async fn execute(
    port: &impl TagCommandPort,
    tenant_id: TenantId,
    name: TagName,
) -> Result<Tag, String> {
    port.ensure_tag(tenant_id, name).await
}
