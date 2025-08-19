//! Contains all serde structs for the Name.com Core API URL Forwarding endpoints.

use serde::{Deserialize, Serialize};

/// Represents a single URL forwarding record.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UrlForwardingRecord {
  pub domain_name: String,
  pub host: String,
  pub forwards_to: String,
  #[serde(rename = "type")]
  pub r#type: String, // "redirect", "masked", or "302"
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<String>,
}

/// The internal response for a paginated list of URL forwarding records.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListUrlForwardingResponse {
  #[serde(rename = "urlForwarding", default)]
  pub forwards: Vec<UrlForwardingRecord>,
  pub next_page: Option<i32>,
}

/// The request body used for creating a new URL forwarding record.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UrlForwardingCreatePayload<'a> {
  pub domain_name: &'a str,
  pub host: &'a str,
  pub forwards_to: &'a str,
  #[serde(rename = "type")]
  pub r#type: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<&'a str>,
}

/// The request body used for updating an existing URL forwarding record.
/// Note that `host` and `domainName` are not included as they are part of the URL path.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UrlForwardingUpdatePayload<'a> {
  pub forwards_to: &'a str,
  #[serde(rename = "type")]
  pub r#type: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<&'a str>,
}
