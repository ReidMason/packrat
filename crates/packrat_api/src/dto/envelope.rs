//! Standard JSON shapes for **all** HTTP response bodies (success and error).
//!
//! # Contract stability
//!
//! The outer keys (`data`, `error`) and inner layout are part of the public API. Changing them is
//! breaking for clients — version the API or coordinate releases.

use serde::Serialize;

/// Successful response: `{ "data": ... }`.
///
/// `T` is usually a DTO from [`crate::dto::assets`] or another resource module.
#[derive(Serialize)]
#[serde(bound = "T: Serialize")]
pub struct SuccessBody<T> {
    pub data: T,
}

impl<T> SuccessBody<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

/// Error payload nested under `error` (see [`ErrorBody`]).
#[derive(Serialize)]
pub struct ErrorEnvelope {
    pub message: String,
}

/// Client error or failure response: `{ "error": { "message": "..." } }`.
#[derive(Serialize)]
pub struct ErrorBody {
    pub error: ErrorEnvelope,
}

impl ErrorBody {
    pub fn message(msg: impl Into<String>) -> Self {
        Self {
            error: ErrorEnvelope {
                message: msg.into(),
            },
        }
    }
}
