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

pub mod database;
pub mod errors;
pub mod http;
pub mod redis;
