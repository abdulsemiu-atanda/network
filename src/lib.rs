//! Public networking and data-access utilities.
//!
//! This crate exposes focused modules for:
//!
//! - Postgres pooling and connection retrieval in [`database`].
//! - Redis key/value operations in [`redis`].
//! - HTTP client helpers in [`http`].
//! - Shared error types in [`errors`].
//!
//! Most public functions return `Result<_, errors::NetworkError>` so callers can
//! use a single error type when composing functionality across modules.

#[cfg(feature = "database")]
pub mod database;
pub mod errors;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "keystore")]
pub mod redis;
#[cfg(feature = "spec")]
pub mod spec_helpers;
