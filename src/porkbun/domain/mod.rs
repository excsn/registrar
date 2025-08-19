//! The Domain sub-client and its methods.

use self::types::{
  DomainCheckResponse, DomainInfo, DomainListRequest, DomainListResponse, GlueRecordIps, GlueRecordListResponse,
  GlueRecordRequest, NameserverListResponse, NameserverUpdateRequest, UrlForwardCreateRequest, UrlForwardListResponse,
  UrlForwardRecord,
};
use super::{
  client::Porkbun,
  endpoints,
  types::{Auth, StatusResponse},
};
use crate::Result;
use std::net::IpAddr;

// Re-export the public types for this module to be used in `porkbun/mod.rs`
pub mod types;

/// Provides access to the Domain functionality of the Porkbun API.
///
/// Created via `Porkbun::domain("example.com")`.
pub struct Domain<'a> {
  client: &'a Porkbun,
  domain: &'a str,
}

impl<'a> Domain<'a> {
  // Constructor is internal to the `porkbun` module.
  pub(super) fn new(client: &'a Porkbun, domain: &'a str) -> Self {
    Self { client, domain }
  }

  /// Updates the nameservers for the domain.
  ///
  /// # Arguments
  /// * `nameservers` - A slice of nameserver hostnames (e.g., `&["ns1.example.com"]`).
  pub async fn update_nameservers(&self, nameservers: &[&str]) -> Result<StatusResponse> {
    let path = format!("{}{}", endpoints::DOMAIN_UPDATE_NS, self.domain);
    let body = NameserverUpdateRequest {
      auth: self.client.auth.clone(),
      ns: nameservers,
    };
    self.client.post(&path, &body).await
  }

  /// Retrieves the authoritative nameservers for the domain.
  pub async fn get_nameservers(&self) -> Result<NameserverListResponse> {
    let path = format!("{}{}", endpoints::DOMAIN_GET_NS, self.domain);
    self.client.post(&path, &self.client.auth).await
  }

  /// Retrieves all domains in your account.
  /// Note: The API returns domains in chunks of 1000. This method handles
  /// pagination internally to return a complete list.
  ///
  /// # Arguments
  /// * `include_labels` - If true, includes any labels assigned to domains.
  pub async fn list_all(&self, include_labels: bool) -> Result<Vec<DomainInfo>> {
    let mut all_domains = Vec::new();
    let mut start = 0;
    loop {
      let body = DomainListRequest {
        auth: self.client.auth.clone(),
        start: Some(start),
        include_labels: if include_labels { Some("yes".to_string()) } else { None },
      };
      let response: DomainListResponse = self.client.post(endpoints::DOMAIN_LIST_ALL, &body).await?;

      if response.domains.is_empty() {
        break;
      }
      start += response.domains.len() as u64;
      all_domains.extend(response.domains);
    }
    Ok(all_domains)
  }

  /// Adds a URL forwarding record.
  ///
  /// # Arguments
  /// * `options` - A reference to a `UrlForwardRecord` containing all configuration.
  pub async fn add_url_forward(&self, options: &UrlForwardRecord) -> Result<StatusResponse> {
    let path = format!("{}{}", endpoints::DOMAIN_ADD_URL_FORWARD, self.domain);
    let body = UrlForwardCreateRequest {
      auth: self.client.auth.clone(),
      subdomain: if options.subdomain.is_empty() {
        None
      } else {
        Some(&options.subdomain)
      },
      location: &options.location,
      r#type: &options.r#type,
      include_path: &options.include_path,
      wildcard: &options.wildcard,
    };
    self.client.post(&path, &body).await
  }

  /// Retrieves all URL forwarding records for the domain.
  pub async fn get_url_forwarding(&self) -> Result<Vec<UrlForwardRecord>> {
    let path = format!("{}{}", endpoints::DOMAIN_GET_URL_FORWARDING, self.domain);
    let response: UrlForwardListResponse = self.client.post(&path, &self.client.auth).await?;
    Ok(response.forwards)
  }

  /// Deletes a specific URL forwarding record by its ID.
  ///
  /// # Arguments
  /// * `record_id` - The numeric ID of the URL forward record to delete.
  pub async fn delete_url_forward(&self, record_id: u64) -> Result<StatusResponse> {
    let path = format!("{}{}/{}", endpoints::DOMAIN_DELETE_URL_FORWARD, self.domain, record_id);
    self.client.post(&path, &self.client.auth).await
  }

  /// Checks the availability of the domain.
  pub async fn check(&self) -> Result<DomainCheckResponse> {
    let path = format!("{}{}", endpoints::DOMAIN_CHECK, self.domain);
    self.client.post(&path, &self.client.auth).await
  }

  /// Creates a glue record for a subdomain of the current domain.
  ///
  /// # Arguments
  /// * `subdomain` - The host part of the glue record (e.g., "ns1").
  /// * `ips` - A slice of IP addresses (v4 or v6) for the glue record.
  pub async fn create_glue_record(&self, subdomain: &str, ips: &[IpAddr]) -> Result<StatusResponse> {
    let path = format!("{}{}/{}", endpoints::DOMAIN_CREATE_GLUE, self.domain, subdomain);
    let body = GlueRecordRequest {
      auth: self.client.auth.clone(),
      ips: ips.to_vec(),
    };
    self.client.post(&path, &body).await
  }

  /// Updates an existing glue record, replacing its IP addresses.
  ///
  /// # Arguments
  /// * `subdomain` - The host part of the glue record to update.
  /// * `ips` - The new slice of IP addresses for the glue record.
  pub async fn update_glue_record(&self, subdomain: &str, ips: &[IpAddr]) -> Result<StatusResponse> {
    let path = format!("{}{}/{}", endpoints::DOMAIN_UPDATE_GLUE, self.domain, subdomain);
    let body = GlueRecordRequest {
      auth: self.client.auth.clone(),
      ips: ips.to_vec(),
    };
    self.client.post(&path, &body).await
  }

  /// Deletes a glue record.
  ///
  /// # Arguments
  /// * `subdomain` - The host part of the glue record to delete.
  pub async fn delete_glue_record(&self, subdomain: &str) -> Result<StatusResponse> {
    let path = format!("{}{}/{}", endpoints::DOMAIN_DELETE_GLUE, self.domain, subdomain);
    self.client.post(&path, &self.client.auth).await
  }

  /// Retrieves all glue records for the domain.
  pub async fn get_glue_records(&self) -> Result<Vec<(String, GlueRecordIps)>> {
    let path = format!("{}{}", endpoints::DOMAIN_GET_GLUE, self.domain);
    let response: GlueRecordListResponse = self.client.post(&path, &self.client.auth).await?;
    Ok(response.hosts)
  }
}
