//! Contains all serde structs for the Porkbun DNS and DNSSEC API endpoints.

use serde::{Deserialize, Serialize};

// --- DNS Record Types ---

/// Represents a single DNS record returned by the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DnsRecord {
  pub id: String,
  pub name: String,
  #[serde(rename = "type")]
  pub r#type: String,
  pub content: String,
  pub ttl: String,
  pub prio: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub notes: Option<String>,
}

/// The response from retrieving a list of DNS records.
#[derive(Deserialize, Debug, Clone)]
pub struct DnsRecordListResponse {
  pub status: String,
  pub records: Vec<DnsRecord>,
}

/// The public-facing options for creating a new DNS record.
/// This struct is provided by the user to the `create_record` method.
#[derive(Debug, Clone)]
pub struct DnsRecordCreateOptions<'a> {
  pub name: Option<&'a str>, // The subdomain
  pub r#type: &'a str,
  pub content: &'a str,
  pub ttl: Option<&'a str>,
  pub prio: Option<&'a str>,
}

/// Options for creating a new DNS record.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DnsRecordCreateRequest<'a> {
  #[serde(flatten)]
  pub auth: super::super::types::Auth,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<&'a str>, // The subdomain
  #[serde(rename = "type")]
  pub r#type: &'a str,
  pub content: &'a str,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ttl: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prio: Option<&'a str>,
}

/// The response after successfully creating a DNS record.
#[derive(Deserialize, Debug, Clone)]
pub struct DnsRecordCreateResponse {
  pub status: String,
  pub id: u64,
}

/// Options for editing a DNS record.
///
/// Use `DnsRecordEditOptions::default()` to create an empty set of options,
/// then set the fields you wish to change.
#[derive(Debug, Clone, Default)]
pub struct DnsRecordEditOptions<'a> {
  pub name: Option<&'a str>,
  pub r#type: Option<&'a str>,
  pub content: Option<&'a str>,
  pub ttl: Option<&'a str>,
  pub prio: Option<&'a str>,
}

/// Options for editing a DNS record by its ID.
/// All fields are optional except for auth.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DnsRecordEditRequest<'a> {
  #[serde(flatten)]
  pub auth: super::super::types::Auth,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<&'a str>,
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub r#type: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ttl: Option<&'a str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prio: Option<&'a str>,
}

// --- DNSSEC Record Types ---

/// Represents a single DNSSEC record.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DnssecRecord {
  pub key_tag: String,
  pub alg: String,
  pub digest_type: String,
  pub digest: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_sig_life: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub key_data_flags: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub key_data_protocol: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub key_data_algo: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub key_data_pub_key: Option<String>,
}

/// Options for creating a DNSSEC record.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DnssecCreateRequest<'a> {
  #[serde(flatten)]
  pub auth: super::super::types::Auth,
  #[serde(flatten)]
  pub record: &'a DnssecRecord,
}

/// The response from retrieving a list of DNSSEC records.
/// The API returns a map where the key is the record's keyTag.
#[derive(Deserialize, Debug, Clone)]
pub struct DnssecRecordListResponse {
  pub status: String,
  pub records: std::collections::HashMap<String, DnssecRecord>,
}
