//! Contains top-level and shared data structures for the Porkbun API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication credentials required for most API calls.
#[derive(Serialize, Clone, Debug)]
pub struct Auth {
  pub secretapikey: String,
  pub apikey: String,
}

/// A generic response indicating the status of an operation.
/// Used by many endpoints that don't return additional data upon success.
#[derive(Deserialize, Debug, Clone)]
pub struct StatusResponse {
  pub status: String,
  #[serde(default)]
  pub message: Option<String>,
}

/// The response for a successful ping request.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
  pub status: String,
  #[serde(rename = "yourIp")]
  pub your_ip: String,
}

/// Represents the TLD pricing information.
#[derive(Deserialize, Debug, Clone)]
pub struct TldPricing {
  pub registration: String,
  pub renewal: String,
  pub transfer: String,
}

/// The response containing pricing for all supported TLDs.
/// The `pricing` field is a map where keys are the TLD (e.g., "com").
#[derive(Deserialize, Debug, Clone)]
pub struct PricingResponse {
  pub status: String,
  pub pricing: HashMap<String, TldPricing>,
}