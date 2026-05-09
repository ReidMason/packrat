use dioxus::prelude::*;

use crate::api_client::AssetDto;

/// Max tag pills on asset list cards (remainder summarized as “+N more”).
pub const ASSET_CARD_TAG_LIMIT: usize = 5;

/// Shared list row for dashboard search and asset browser.
#[component]
pub fn AssetCard(asset: AssetDto) -> Element {
    let parent_note = if asset.parent_id.is_some() {
        "Nested under another asset"
    } else {
        "Top level"
    };
    let tag_extra = asset.tags.len().saturating_sub(ASSET_CARD_TAG_LIMIT);
    rsx! {
        div {
            class: "rounded-lg border border-ui-bg-dim bg-ui-bg-dim/40 p-4 space-y-2 text-sm cursor-pointer hover:opacity-95 transition-opacity",
            p { class: "text-base font-medium text-ui-text", "{asset.name}" }
            dl {
                class: "grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-ui-text-muted text-xs",
                dt { "Placement" }
                dd { "{parent_note}" }
                dt { "Created" }
                dd { class: "font-mono", "{asset.created}" }
                if asset.deleted.is_some() {
                    dt { "Status" }
                    dd { "Removed" }
                }
            }
            if !asset.tags.is_empty() {
                div {
                    class: "flex flex-wrap gap-1.5 items-center pt-2",
                    for tag in asset.tags.iter().take(ASSET_CARD_TAG_LIMIT) {
                        span {
                            key: "{tag.id}",
                            class: "rounded-full bg-ui-bg-dim/80 border border-ui-bg-dim px-2 py-0.5 text-[11px] font-medium text-ui-text-muted",
                            "{tag.name}"
                        }
                    }
                    if tag_extra > 0 {
                        span {
                            class: "rounded-full border border-dashed border-ui-bg-dim px-2 py-0.5 text-[11px] font-medium text-ui-text-dim",
                            "+{tag_extra} more"
                        }
                    }
                }
            }
        }
    }
}
