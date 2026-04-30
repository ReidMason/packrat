use crate::ports::ReadinessPort;

pub async fn check_readiness(port: &impl ReadinessPort) -> Result<(), String> {
    port.check_database().await
}
