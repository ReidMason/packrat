//! HTTP **DTOs** and **response envelopes** — the public JSON contract.
//!
//! # Danger: external API surface
//!
//! - **DTOs** (`items`, …): request/response *payload* shapes.
//! - **Envelopes** ([`SuccessBody`], [`ErrorBody`]): how those payloads are wrapped.
//!
//! Renames, new fields, or envelope changes are **breaking** for Postman, scripts, and apps unless
//! you version routes or coordinate clients.

mod envelope;
mod items;

pub use envelope::{ErrorBody, SuccessBody};
pub use items::{CreateItemDto, ItemDto};