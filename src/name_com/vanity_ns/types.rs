//! Contains all serde structs for the Name.com Core API Vanity Nameserver endpoints.

use serde::{Deserialize, Serialize};

/// Represents a single vanity nameserver.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VanityNameserver {
  pub domain_name: String,
  pub hostname: String,
  #[serde(default)]
  pub ips: Vec<String>,
}

/// The internal response for a paginated list of vanity nameservers.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListVanityNsResponse {
  #[serde(rename = "vanityNameservers", default)]
  pub vanity_ns: Vec<VanityNameserver>,
  pub next_page: Option<i32>,
}

/// The request body used for creating a new vanity nameserver.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VanityNsCreatePayload<'a> {
  pub hostname: &'a str,
  pub ips: Vec<&'a str>,
}

/// The request body used for updating an existing vanity nameserver.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VanityNsUpdatePayload<'a> {
  pub ips: Vec<&'a str>,
}
