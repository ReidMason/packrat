use packrat_domain::tenant::Tenant;
use serde::Serialize;

/// JSON body for `POST /api/tenants`.
#[derive(serde::Deserialize)]
pub struct CreateTenantDto {
    pub name: String,
}

#[derive(Serialize)]
pub struct TenantDto {
    pub id: i64,
    pub name: String,
    pub created: String,
    pub updated: String,
}

impl TenantDto {
    pub fn from_tenant(tenant: Tenant) -> Self {
        Self {
            id: i64::from(tenant.id),
            name: tenant.name.as_str().to_string(),
            created: tenant.created.to_string(),
            updated: tenant.updated.to_string(),
        }
    }
}
