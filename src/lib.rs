//! # handl-lib-crypto
//!
//! Cryptographic hashing for the handl runtime — **pure `.hnl`**.
//!
//! `crypto.sha1`/`crypto.sha256`/`crypto.md5` are Handl arrows in
//! `handl-packages/crypto/crypto.hnl` over the `math`/`bytes`/`typeof`
//! substrate ops (u32 arithmetic, `rotl32`, `bit_*`). This crate is a data
//! carrier: it exports the `.hnl` package source so `stdlib::MODULES` can
//! register `crypto` as a table module.

/// The handl source declaring the crypto `op` interface plus the pure-handl
/// digest implementations — the single source of truth for the crypto API.
pub const CRYPTO_HNL: &str = include_str!("../handl-packages/crypto/crypto.hnl");

/// The exported `.hnl` arrows (each `Value -> String` hex digest).
pub const CRYPTO_FUNCS: &[&str] = &["sha1", "sha256", "md5"];
