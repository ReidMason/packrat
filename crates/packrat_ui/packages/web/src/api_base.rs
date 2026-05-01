//! Default API base URL. On WASM, `initial_api_base` may read a previously stored value from
//! `localStorage` (key `packrat_api_base_v1`) so older sessions keep working; the UI no longer writes it.

pub const DEFAULT_API_BASE: &str = "http://127.0.0.1:3000";

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
const STORAGE_KEY: &str = "packrat_api_base_v1";

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
