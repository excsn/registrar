//! # Registrar
//!
//! A unified client for domain registrar APIs.
//!
//! This crate provides a strongly-typed, asynchronous interface to interact with
//! various domain registrar APIs. Each registrar is enabled via a feature flag
//! to keep the crate lightweight.
//!
//! ## Currently Supported Registrars
//!
//! - `porkbun` (requires the "porkbun" feature)
//! - `name-com` (requires the "name-com" feature)
//!

use thiserror::Error;

/// A universal error type for all registrar operations.
///
/// This enum consolidates errors from the underlying HTTP client, JSON
/// serialization/deserialization, and specific API error messages
/// returned by the registrar.
#[derive(Error, Debug)]
pub enum Error {
  /// An error occurred during the HTTP request. This could be a network issue,
  /// a DNS problem, or an invalid certificate.
  #[error("HTTP request failed: {0}")]
  Http(#[from] reqwest::Error),

  /// An error occurred while serializing a request to JSON or deserializing
  /// a response from JSON.
  #[error("Failed to parse JSON: {0}")]
  Json(#[from] serde_json::Error),

  /// The registrar's API returned a specific error message (e.g., "Invalid API Key").
  #[error("API Error: {0}")]
  Api(String),
}

/// A specialized `Result` type for registrar operations.
///
/// This type alias simplifies function signatures throughout the crate
/// by using `registrar::Error` as the default error type.
pub type Result<T> = std::result::Result<T, Error>;

// Conditionally compile and expose the porkbun module.
// This block of code will only be included if the "porkbun" feature
// is enabled by the user of this crate.
#[cfg(feature = "porkbun")]
pub mod porkbun;

// This block of code will only be included if the "name-com" feature
// is enabled by the user of this crate.
#[cfg(feature = "name-com")]
pub mod name_com;