//! Public networking and data-access utilities.
//!
//! This crate exposes focused modules for:
//!
//! - Postgres pooling and connection retrieval in the `database` module.
//! - Redis key/value operations in the `redis` module.
//! - HTTP client helpers in the `http` module.
//! - Transactional database test setup helpers in the `spec_helpers` module.
//! - Shared error types in [`errors`].
//!
//! Most public functions return `Result<_, errors::NetworkError>` so callers can
//! use a single error type when composing functionality across modules.
//!
//! Feature flags:
//!
//! - `http` enables the `http` module and is part of the default feature set.
//! - `database` enables the `database` module.
//! - `keystore` enables the `redis` module.
//! - `spec` enables the `spec_helpers` module and also enables `database`.

#[cfg(feature = "database")]
pub mod database;
pub mod errors;
#[cfg(feature = "http")]
pub mod http;
pub use reqwest::{Error as ReqwestError, header};
#[cfg(feature = "keystore")]
pub mod redis;
#[cfg(feature = "spec")]
pub mod spec_helpers;
