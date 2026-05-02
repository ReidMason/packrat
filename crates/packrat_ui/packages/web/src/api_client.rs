//! HTTP client for the Packrat Axum API (`/api/*`).
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct SuccessBody<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
struct ErrorEnvelope {
    message: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ErrorBody {
    error: ErrorEnvelope,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HealthDto {
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ReadyDto {
    pub status: String,
    pub database: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AssetDto {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub created: String,
    pub deleted: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateAssetRequest {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<i64>,
}

fn normalize_base(base: &str) -> String {
    base.trim().trim_end_matches('/').to_string()
}

fn http_base(configured: &str) -> String {
    let b = normalize_base(configured);
    if !b.is_empty() {
        return b;
    }
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "http://127.0.0.1:3000".to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "http://127.0.0.1:3000".to_string()
    }
}

async fn map_api_error(resp: reqwest::Response) -> String {
    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();
    if let Ok(body) = serde_json::from_str::<ErrorBody>(&text) {
        return format!("{status}: {}", body.error.message);
    }
    if text.is_empty() {
        status.to_string()
    } else {
        format!("{status}: {text}")
    }
}

pub async fn fetch_health(base: &str) -> Result<HealthDto, String> {
    let url = format!("{}/api/health", http_base(base));
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<HealthDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn fetch_ready(base: &str) -> Result<ReadyDto, String> {
    let url = format!("{}/api/ready", http_base(base));
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<ReadyDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

/// Liveness and readiness in one round-trip pair (for the dashboard).
pub async fn fetch_api_status(base: &str) -> (Result<HealthDto, String>, Result<ReadyDto, String>) {
    let h = fetch_health(base).await;
    let r = fetch_ready(base).await;
    (h, r)
}

#[derive(Debug, Serialize)]
struct SearchAssetsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzyname: Option<String>,
}

pub async fn search_assets(base: &str, fuzzyname: &str) -> Result<Vec<AssetDto>, String> {
    let needle = fuzzyname.trim();
    if needle.is_empty() {
        return Err("Search text must not be empty.".into());
    }
    let url = format!("{}/api/assets/search", http_base(base));
    let body = SearchAssetsRequest {
        name: None,
        fuzzyname: Some(needle.to_string()),
    };
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let wrapped: SuccessBody<Vec<AssetDto>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}

pub async fn list_assets(base: &str) -> Result<Vec<AssetDto>, String> {
    let url = format!("{}/api/assets", http_base(base));
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<Vec<AssetDto>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn get_asset(base: &str, id: i64) -> Result<AssetDto, String> {
    let url = format!("{}/api/assets/{id}", http_base(base));
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<AssetDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn list_child_assets(base: &str, parent_id: i64) -> Result<Vec<AssetDto>, String> {
    let url = format!("{}/api/assets/{parent_id}/children", http_base(base));
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<Vec<AssetDto>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn create_asset(
    base: &str,
    name: String,
    parent_id: Option<i64>,
) -> Result<AssetDto, String> {
    let url = format!("{}/api/assets", http_base(base));
    let body = CreateAssetRequest { name, parent_id };
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let wrapped: SuccessBody<AssetDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}

pub async fn delete_asset(base: &str, id: i64) -> Result<(), String> {
    let url = format!("{}/api/assets/{id}", http_base(base));
    let resp = reqwest::Client::new()
        .delete(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if resp.status() == reqwest::StatusCode::NO_CONTENT {
        return Ok(());
    }
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    Ok(())
}
