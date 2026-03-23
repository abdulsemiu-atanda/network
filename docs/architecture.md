# Architecture

## Overview

This crate provides thin wrappers around three integration concerns:

- Postgres access through Diesel pooling.
- Redis key/value operations with namespacing and expiry.
- HTTP requests through reqwest.

Each module is intentionally small and focused. The crate does not attempt to be a full framework; it standardizes repeated setup and error handling patterns.

## Module boundaries

- `database`: Creates and owns Diesel `r2d2` pools. It exposes helper constructors and pooled connection retrieval.
- `redis`: Wraps a Redis connection with namespaced key operations (`retrieve`, `insert`, `delete`).
- `http`: Wraps `reqwest::Client` and provides method-specific helpers (`get`, `post`, `patch`, `put`).
- `errors`: Defines crate-wide error enums and conversion paths.

## Dependency rationale

- `diesel` with `r2d2`: established Postgres ORM + connection pooling model.
- `redis`: direct command API for lightweight key/value interactions.
- `reqwest`: async HTTP client with JSON support.

## Data flow model

1. Callers create or receive clients/pools from module constructors.
2. Callers invoke operation methods in each module.
3. Library converts lower-level errors into `NetworkError` variants.
4. Application code handles one top-level error type across integrations.

## Design constraints

- Keep wrappers explicit and readable over highly abstract APIs.
- Avoid hiding side effects (network, cache, DB) behind implicit behavior.
- Keep signatures aligned with underlying crates where practical.
