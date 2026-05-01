use dioxus::prelude::*;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub const RECENT_KEY: &str = "packrat_recent_v1";
pub const MAX_RECENT: usize = 10;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecentBrief {
    pub id: i64,
    pub name: String,
}

#[cfg(target_arch = "wasm32")]
pub fn load_recent_disk() -> Vec<RecentBrief> {
    try_load_recent().unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn try_load_recent() -> Option<Vec<RecentBrief>> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let json = storage.get_item(RECENT_KEY).ok().flatten()?;
    serde_json::from_str(&json).ok()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_recent_disk() -> Vec<RecentBrief> {
    Vec::new()
}

#[cfg(target_arch = "wasm32")]
pub fn save_recent_disk(entries: &[RecentBrief]) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(json) = serde_json::to_string(entries) {
                let _ = storage.set_item(RECENT_KEY, &json);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_recent_disk(_entries: &[RecentBrief]) {}

pub fn remember_recent(mut recent: Signal<Vec<RecentBrief>>, id: i64, name: String) {
    let mut v = recent();
    v.retain(|e| e.id != id);
    v.insert(0, RecentBrief { id, name });
    v.truncate(MAX_RECENT);
    save_recent_disk(&v);
    recent.set(v);
}
