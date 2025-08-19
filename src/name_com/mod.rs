//! # Name.com API Client
//!
//! Provides a client and all necessary types for interacting with the
//! Name.com Core V1 API.
//!
//! To use this module, you must enable the `name-com` feature in your `Cargo.toml`.
//!
//! ## Example
//!
//! ```no_run
//! use registrar::name_com::NameDotCom;
//!
//! let username = "YOUR_USERNAME".to_string();
//! let token = "YOUR_API_TOKEN".to_string();
//! let client = NameDotCom::new(username, token);
//! ```

pub mod client;
pub mod dns;
pub mod domain;
pub mod endpoints;
pub mod types;
pub mod url_forwarding;
pub mod vanity_ns;

pub use client::NameDotCom;