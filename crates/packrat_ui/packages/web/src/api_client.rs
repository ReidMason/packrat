//! HTTP client for the Packrat Axum API (`/api/*`).
use serde::{Deserialize, Serialize};

use super::api_base::DEFAULT_API_BASE;

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
pub struct ItemDto {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub created: String,
    pub deleted: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateItemRequest {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<i64>,
}

fn normalize_base(base: &str) -> String {
    let t = base.trim().trim_end_matches('/');
    if t.is_empty() {
        return DEFAULT_API_BASE.trim().trim_end_matches('/').to_string();
    }
    t.to_string()
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
    let url = format!("{}/api/health", normalize_base(base));
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
    let url = format!("{}/api/ready", normalize_base(base));
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
struct SearchItemsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzzyname: Option<String>,
}

pub async fn search_items(base: &str, fuzzyname: &str) -> Result<Vec<ItemDto>, String> {
    let needle = fuzzyname.trim();
    if needle.is_empty() {
        return Err("Search text must not be empty.".into());
    }
    let url = format!("{}/api/items/search", normalize_base(base));
    let body = SearchItemsRequest {
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
    let wrapped: SuccessBody<Vec<ItemDto>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}

pub async fn list_items(base: &str) -> Result<Vec<ItemDto>, String> {
    let url = format!("{}/api/items", normalize_base(base));
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<Vec<ItemDto>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn get_item(base: &str, id: i64) -> Result<ItemDto, String> {
    let url = format!("{}/api/items/{id}", normalize_base(base));
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<ItemDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn list_child_items(base: &str, parent_id: i64) -> Result<Vec<ItemDto>, String> {
    let url = format!("{}/api/items/{parent_id}/children", normalize_base(base));
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<Vec<ItemDto>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn create_item(base: &str, name: String, parent_id: Option<i64>) -> Result<ItemDto, String> {
    let url = format!("{}/api/items", normalize_base(base));
    let body = CreateItemRequest { name, parent_id };
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let wrapped: SuccessBody<ItemDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}

pub async fn delete_item(base: &str, id: i64) -> Result<(), String> {
    let url = format!("{}/api/items/{id}", normalize_base(base));
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
