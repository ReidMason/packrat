use async_trait::async_trait;

#[async_trait]
pub trait ReadinessPort: Send + Sync {
    async fn check_database(&self) -> Result<(), String>;
}
