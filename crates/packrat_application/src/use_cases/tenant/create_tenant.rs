use packrat_domain::tenant::{Tenant, TenantName};

use crate::ports::{TenantCommandError, TenantCommandPort};

pub async fn execute(
    port: &impl TenantCommandPort,
    name: TenantName,
) -> Result<Tenant, TenantCommandError> {
    port.create_tenant(name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockTenantCommand;

    #[async_trait]
    impl TenantCommandPort for MockTenantCommand {
        async fn create_tenant(&self, name: TenantName) -> Result<Tenant, TenantCommandError> {
            Ok(Tenant::new(
                packrat_domain::tenant::TenantId::from(1),
                name,
                packrat_domain::entity::EntityTimestamp::static_for_tests(),
                packrat_domain::entity::EntityTimestamp::static_for_tests(),
            ))
        }
    }

    #[tokio::test]
    async fn execute_delegates_to_port() {
        let port = MockTenantCommand;
        let tenant = execute(&port, TenantName::from("Acme")).await.unwrap();
        assert_eq!(tenant.name.as_str(), "Acme");
        assert_eq!(i64::from(tenant.id), 1);
    }
}
