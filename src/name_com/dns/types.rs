//! Contains all serde structs for the Name.com Core API DNS endpoints.

use serde::{Deserialize, Serialize};

// =================================================================================
// Core DNS Record Types
// =================================================================================

/// Represents a single DNS record as returned by the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecord {
  pub id: i32,
  pub domain_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub host: Option<String>,
  pub fqdn: String,
  #[serde(rename = "type")]
  pub r#type: String,
  pub answer: String,
  pub ttl: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub priority: Option<i64>,
}

/// The response for a paginated list of DNS records.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListDnsRecordsResponse {
  #[serde(default)]
  pub records: Vec<DnsRecord>,
  pub next_page: Option<i32>,
  pub last_page: Option<i32>,
}

/// The request body used for creating or updating a DNS record.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecordPayload<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub host: Option<&'a str>,
  #[serde(rename = "type")]
  pub r#type: &'a str,
  pub answer: &'a str,
  pub ttl: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub priority: Option<i64>,
}

// =================================================================================
// DNSSEC Record Types
// =================================================================================

/// Represents a single DNSSEC record.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DnssecRecord {
  pub domain_name: String,
  pub digest: String,
  #[serde(rename = "digestType")]
  pub digest_type: i32,
  #[serde(rename = "keyTag")]
  pub key_tag: i32,
  pub algorithm: i32,
}

/// The internal response for a list of DNSSEC records.
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ListDnssecResponse {
  #[serde(default)]
  pub dnssec: Vec<DnssecRecord>,
}

/// The request body used for creating a new DNSSEC record.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DnssecCreatePayload<'a> {
  pub digest: &'a str,
  #[serde(rename = "digestType")]
  pub digest_type: i32,
  #[serde(rename = "keyTag")]
  pub key_tag: i32,
  pub algorithm: i32,
}