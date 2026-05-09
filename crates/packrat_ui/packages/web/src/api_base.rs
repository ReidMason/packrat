#[cfg(target_arch = "wasm32")]
pub const DEFAULT_API_BASE: &str = "";

#[cfg(not(target_arch = "wasm32"))]
pub const DEFAULT_API_BASE: &str = "http://127.0.0.1:3000";

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
const STORAGE_KEY: &str = "packrat_api_base_v1";

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
const TENANT_ID_KEY: &str = "packrat_tenant_id_v1";

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
const AUTH_TOKEN_KEY: &str = "packrat_auth_token_v1";

pub fn initial_auth_token() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        load_stored_auth_token()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn load_stored_auth_token() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let v = storage.get_item(AUTH_TOKEN_KEY).ok().flatten()?;
    let t = v.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
pub fn persist_auth_token(token: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(AUTH_TOKEN_KEY, token.trim());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn persist_auth_token(_token: &str) {}

#[cfg(target_arch = "wasm32")]
pub fn clear_auth_token() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(AUTH_TOKEN_KEY);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn clear_auth_token() {}

pub fn initial_tenant_id() -> Option<i64> {
    #[cfg(target_arch = "wasm32")]
    {
        load_stored_tenant_id()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn load_stored_tenant_id() -> Option<i64> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let v = storage.get_item(TENANT_ID_KEY).ok().flatten()?;
    v.trim().parse().ok()
}

#[cfg(target_arch = "wasm32")]
pub fn persist_tenant_id(id: i64) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(TENANT_ID_KEY, &id.to_string());
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn persist_tenant_id(_id: i64) {}

#[cfg(target_arch = "wasm32")]
pub fn clear_tenant_id() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(TENANT_ID_KEY);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn clear_tenant_id() {}

pub fn initial_api_base() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        load_stored_api_base().unwrap_or_else(|| DEFAULT_API_BASE.to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        DEFAULT_API_BASE.to_string()
    }
}

#[cfg(target_arch = "wasm32")]
fn load_stored_api_base() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let v = storage.get_item(STORAGE_KEY).ok().flatten()?;
    let t = v.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}
