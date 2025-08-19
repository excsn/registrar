//! # Porkbun API Client
//!
//! Provides a client and all necessary types for interacting with the
//! Porkbun v3 API.
//!
//! To use this module, you must enable the `porkbun` feature in your `Cargo.toml`.
//!
//! ## Example Usage (Porkbun)
//!
//! ```no_run
//! // This example requires the "porkbun" feature to be enabled.
//! use registrar::porkbun::Porkbun;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), registrar::Error> {
//!   let apikey = std::env::var("PORKBUN_API_KEY").expect("PORKBUN_API_KEY not set");
//!   let secret = std::env::var("PORKBUN_SECRET_KEY").expect("PORKBUN_SECRET_KEY not set");
//!
//!   let client = Porkbun::new(apikey, secret);
//!
//!   // Ping the API to test credentials
//!   let response = client.ping().await?;
//!   println!("Successfully connected. Your IP is: {}", response.your_ip);
//!
//!   // Get all DNS records for a domain
//!   let records = client.dns("example.com").retrieve_all_records().await?;
//!   println!("Found {} DNS records for example.com", records.len());
//!
//!   Ok(())
//! }
//! ```

pub mod client;
pub mod dns;
pub mod domain;
pub mod endpoints;
pub mod ssl;
pub mod types;

pub use client::Porkbun;