//! Contains all serde structs for the Porkbun Domain API endpoints.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;

// --- Nameserver Types ---

/// Request body for updating nameservers.
#[derive(Serialize, Debug, Clone)]
pub(crate) struct NameserverUpdateRequest<'a> {
  #[serde(flatten)]
  pub auth: super::super::types::Auth,
  pub ns: &'a [&'a str],
}

/// Response for a request to get nameservers.
#[derive(Deserialize, Debug, Clone)]
pub struct NameserverListResponse {
  pub status: String,
  pub ns: Vec<String>,
}

// --- Domain Listing Types ---

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DomainListRequest {
  #[serde(flatten)]
  pub auth: super::super::types::Auth,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub include_labels: Option<String>, // "yes" or "no"
}

/// A label associated with a domain.
#[derive(Deserialize, Debug, Clone)]
pub struct Label {
  pub id: String,
  pub title: String,
  pub color: String,
}

/// Detailed information about a single domain in an account.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DomainInfo {
  pub domain: String,
  pub status: String,
  pub tld: String,
  pub create_date: String,
  pub expire_date: String,
  pub security_lock: String, // "1" or "0"
  pub whois_privacy: String, // "1" or "0"
  pub auto_renew: u8,        // 1 or 0
  pub not_local: u8,         // 1 or 0
  #[serde(default)]
  pub labels: Vec<Label>,
}

/// The response containing a list of domains.
#[derive(Deserialize, Debug, Clone)]
pub struct DomainListResponse {
  pub status: String,
  pub domains: Vec<DomainInfo>,
}

// --- URL Forwarding Types ---

/// A URL forwarding record.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UrlForwardRecord {
  pub id: String,
  pub subdomain: String,
  pub location: String,
  pub r#type: String,
  pub include_path: String,
  pub wildcard: String,
}

/// The response containing a list of URL forwarding records.
#[derive(Deserialize, Debug, Clone)]
pub struct UrlForwardListResponse {
  pub status: String,
  pub forwards: Vec<UrlForwardRecord>,
}

/// Options for creating a new URL forwarding record.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UrlForwardCreateRequest<'a> {
  #[serde(flatten)]
  pub auth: super::super::types::Auth,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subdomain: Option<&'a str>,
  pub location: &'a str,
  pub r#type: &'a str,       // "temporary" or "permanent"
  pub include_path: &'a str, // "yes" or "no"
  pub wildcard: &'a str,     // "yes" or "no"
}

// --- Domain Check Types ---

#[derive(Deserialize, Debug, Clone)]
pub struct PriceInfo {
  pub r#type: String,
  pub price: String,
  #[serde(rename = "regularPrice")]
  pub regular_price: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AdditionalPricing {
  pub renewal: PriceInfo,
  pub transfer: PriceInfo,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DomainAvailability {
  pub avail: String, // "yes" or "no"
  pub r#type: String,
  pub price: String,
  pub first_year_promo: String, // "yes" or "no"
  pub regular_price: String,
  pub premium: String, // "yes" or "no"
  pub additional: AdditionalPricing,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitInfo {
  #[serde(rename = "TTL")]
  pub ttl: String,
  pub limit: String,
  pub used: u64,
  pub natural_language: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DomainCheckResponse {
  pub status: String,
  pub response: DomainAvailability,
  pub limits: RateLimitInfo,
}

// --- Glue Record Types ---

/// Request body for creating or updating a glue record.
#[derive(Serialize, Debug, Clone)]
pub(crate) struct GlueRecordRequest {
  #[serde(flatten)]
  pub auth: super::super::types::Auth,
  pub ips: Vec<IpAddr>,
}

/// Represents the v4 and v6 IPs for a glue record host.
#[derive(Deserialize, Debug, Clone)]
pub struct GlueRecordIps {
  #[serde(default)]
  pub v6: Vec<IpAddr>,
  #[serde(default)]
  pub v4: Vec<IpAddr>,
}

/// The response containing a list of glue records for a domain.
#[derive(Deserialize, Debug, Clone)]
pub struct GlueRecordListResponse {
  pub status: String,
  // The API returns an array of [hostname, ips_object] tuples.
  pub hosts: Vec<(String, GlueRecordIps)>,
}
