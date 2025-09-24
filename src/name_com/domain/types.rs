//! Contains all serde structs for the Name.com Core API Domain endpoints.

use serde::{Deserialize, Serialize};
use serde_json::Value; // For the complex/unspecified Contact struct

// A placeholder for the detailed contact info.
// In a full implementation, this would be a detailed struct.
pub type Contact = Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contacts {
  pub registrant: Contact,
  pub admin: Contact,
  pub tech: Contact,
  pub billing: Contact,
}

/// Represents a single domain in a Name.com account.
/// This is used as a response payload from most domain endpoints.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
  pub domain_name: String,
  pub create_date: String,
  pub expire_date: String,
  pub autorenew_enabled: bool,
  pub locked: bool,
  pub privacy_enabled: bool,
  pub contacts: Contacts,
  pub nameservers: Vec<String>,
  pub renewal_price: Option<f64>,
}

/// Represents the response from a ListDomains request.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListDomainsResponse {
  #[serde(default)]
  pub domains: Vec<Domain>,
  pub next_page: Option<i32>,
  pub last_page: Option<i32>,
}

/// The inner payload for creating a new domain.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DomainCreatePayload<'a> {
  pub domain_name: &'a str,
}

/// The top-level request body for creating a new domain.
#[derive(Serialize, Debug, Clone)]
pub(crate) struct CreateDomainRequest<'a> {
  pub domain: DomainCreatePayload<'a>,
}

/// The response after successfully creating a domain.
#[derive(Deserialize, Debug, Clone)]
pub struct CreateDomainResponse {
  pub domain: Domain,
  pub order: i32,
  #[serde(rename = "totalPaid")]
  pub total_paid: f64,
}

/// Request body for updating a domain's lock/autorenew/privacy status.
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDomainPayload {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub autorenew_enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub locked: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub privacy_enabled: Option<bool>,
}

/// Request body for setting nameservers for a domain.
#[derive(Serialize, Debug, Clone)]
pub(crate) struct SetNameserversRequest {
  pub nameservers: Vec<String>,
}

/// Response for a GetAuthCode request.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAuthCodeResponse {
  pub auth_code: String,
}

/// Request body for checking the availability of one or more domain names.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CheckAvailabilityRequest {
  pub domain_names: Vec<String>,
}

/// The result of an availability check for a single domain.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityResult {
  pub domain_name: String,
  pub purchasable: bool,
  pub premium: bool,
  pub purchase_price: f64,
  pub purchase_type: String,
  pub renewal_price: f64,
}

/// The response from a domain availability check.
#[derive(Deserialize, Debug, Clone)]
pub struct CheckAvailabilityResponse {
  pub results: Vec<AvailabilityResult>,
}