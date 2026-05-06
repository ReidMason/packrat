use async_trait::async_trait;
use packrat_domain::tenant::{Tenant, TenantName};

#[derive(Debug, PartialEq, Eq)]
pub enum TenantCommandError {
    Persist(String),
}

impl std::fmt::Display for TenantCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TenantCommandError::Persist(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for TenantCommandError {}

#[async_trait]
pub trait TenantCommandPort: Send + Sync {
    async fn create_tenant(&self, name: TenantName) -> Result<Tenant, TenantCommandError>;
}
