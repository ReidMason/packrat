//! HTTP client for the Packrat Axum API (`/api/*`).
use reqwest::header::AUTHORIZATION;
use reqwest::RequestBuilder;
use serde::{Deserialize, Serialize};

fn with_bearer(req: RequestBuilder, token: Option<&str>) -> RequestBuilder {
    if let Some(t) = token.map(str::trim).filter(|s| !s.is_empty()) {
        req.header(AUTHORIZATION, format!("Bearer {t}"))
    } else {
        req
    }
}

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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserDto {
    pub id: i64,
    pub email: String,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TenantDto {
    pub id: i64,
    pub name: String,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct LoginDto {
    pub user_id: i64,
    pub token: String,
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

#[cfg(target_arch = "wasm32")]
fn wasm_default_http_base() -> String {
    let Some(win) = web_sys::window() else {
        return "http://127.0.0.1:3000".to_string();
    };
    let Ok(origin) = win.location().origin() else {
        return "http://127.0.0.1:3000".to_string();
    };
    let origin = origin.trim_end_matches('/').to_string();
    if origin.is_empty() {
        return "http://127.0.0.1:3000".to_string();
    }
    let host_lc = win
        .location()
        .hostname()
        .ok()
        .unwrap_or_default()
        .to_lowercase();
    let port = win.location().port().unwrap_or_default();
    let local = host_lc == "localhost"
        || host_lc == "127.0.0.1"
        || host_lc == "[::1]"
        || host_lc.ends_with(".localhost");
    if local && !port.is_empty() && port != "3000" {
        return "http://127.0.0.1:3000".to_string();
    }
    origin
}

fn http_base(configured: &str) -> String {
    let b = normalize_base(configured);
    if !b.is_empty() {
        return b;
    }
    #[cfg(target_arch = "wasm32")]
    {
        wasm_default_http_base()
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

pub async fn search_assets(
    base: &str,
    fuzzyname: &str,
    token: Option<&str>,
) -> Result<Vec<AssetDto>, String> {
    let needle = fuzzyname.trim();
    if needle.is_empty() {
        return Err("Search text must not be empty.".into());
    }
    let url = format!("{}/api/assets/search", http_base(base));
    let body = SearchAssetsRequest {
        name: None,
        fuzzyname: Some(needle.to_string()),
    };
    let resp = with_bearer(reqwest::Client::new().post(&url).json(&body), token)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let wrapped: SuccessBody<Vec<AssetDto>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}

pub async fn list_assets(base: &str, token: Option<&str>) -> Result<Vec<AssetDto>, String> {
    let url = format!("{}/api/assets", http_base(base));
    let resp = with_bearer(reqwest::Client::new().get(&url), token)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<Vec<AssetDto>> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn get_asset(base: &str, id: i64, token: Option<&str>) -> Result<AssetDto, String> {
    let url = format!("{}/api/assets/{id}", http_base(base));
    let resp = with_bearer(reqwest::Client::new().get(&url), token)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let body: SuccessBody<AssetDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(body.data)
}

pub async fn list_child_assets(
    base: &str,
    parent_id: i64,
    token: Option<&str>,
) -> Result<Vec<AssetDto>, String> {
    let url = format!("{}/api/assets/{parent_id}/children", http_base(base));
    let resp = with_bearer(reqwest::Client::new().get(&url), token)
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
    token: Option<&str>,
) -> Result<AssetDto, String> {
    let url = format!("{}/api/assets", http_base(base));
    let body = CreateAssetRequest { name, parent_id };
    let resp = with_bearer(reqwest::Client::new().post(&url).json(&body), token)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let wrapped: SuccessBody<AssetDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}

pub async fn delete_asset(base: &str, id: i64, token: Option<&str>) -> Result<(), String> {
    let url = format!("{}/api/assets/{id}", http_base(base));
    let resp = with_bearer(reqwest::Client::new().delete(&url), token)
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

#[derive(Debug, Serialize)]
struct CreateUserRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct CreateTenantRequest {
    name: String,
}

pub async fn create_user(
    base: &str,
    email: String,
    password: String,
) -> Result<UserDto, String> {
    let url = format!("{}/api/users", http_base(base));
    let body = CreateUserRequest { email, password };
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let wrapped: SuccessBody<UserDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}

pub async fn login(base: &str, email: String, password: String) -> Result<LoginDto, String> {
    let url = format!("{}/api/login", http_base(base));
    let body = LoginRequest { email, password };
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let wrapped: SuccessBody<LoginDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}

pub async fn create_tenant(
    base: &str,
    name: String,
    token: Option<&str>,
) -> Result<TenantDto, String> {
    let url = format!("{}/api/tenants", http_base(base));
    let body = CreateTenantRequest { name };
    let resp = with_bearer(reqwest::Client::new().post(&url).json(&body), token)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(map_api_error(resp).await);
    }
    let wrapped: SuccessBody<TenantDto> = resp.json().await.map_err(|e| e.to_string())?;
    Ok(wrapped.data)
}
