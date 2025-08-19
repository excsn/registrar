//! The Dns sub-client and its methods.

use self::types::{
  DnsRecord, DnsRecordCreateRequest, DnsRecordCreateResponse, DnsRecordEditRequest, DnsRecordListResponse,
  DnssecCreateRequest, DnssecRecord, DnssecRecordListResponse,
};
use super::{client::Porkbun, endpoints, types::StatusResponse};
use crate::{porkbun::dns::types::{DnsRecordCreateOptions, DnsRecordEditOptions}, Result};

// Re-export the public types for this module to be used in `porkbun/mod.rs`
pub mod types;

/// Provides access to the DNS functionality of the Porkbun API.
///
/// Created via `Porkbun::dns("example.com")`.
pub struct Dns<'a> {
  client: &'a Porkbun,
  domain: &'a str,
}

impl<'a> Dns<'a> {
  // Constructor is internal to the `porkbun` module.
  pub(super) fn new(client: &'a Porkbun, domain: &'a str) -> Self {
    Self { client, domain }
  }

  /// Creates a new DNS record.
  ///
  /// # Arguments
  /// * `options` - A struct containing all configuration for the new record.
  pub async fn create_record(&self, options: DnsRecordCreateOptions<'_>) -> Result<DnsRecordCreateResponse> {
    let path = format!("{}{}", endpoints::DNS_CREATE, self.domain);
    // Build the internal request struct using the user's options and the client's auth.
    let body = DnsRecordCreateRequest {
      auth: self.client.auth.clone(),
      name: options.name,
      r#type: options.r#type,
      content: options.content,
      ttl: options.ttl,
      prio: options.prio,
    };
    self.client.post(&path, &body).await
  }

  /// Edits a specific DNS record by its ID.
  ///
  /// # Arguments
  /// * `record_id` - The numeric ID of the DNS record to edit.
  /// * `options` - A struct containing the fields to change.
  pub async fn edit_record_by_id(&self, record_id: u64, options: DnsRecordEditOptions<'_>) -> Result<StatusResponse> {
    let path = format!("{}{}/{}", endpoints::DNS_EDIT_BY_ID, self.domain, record_id);
    // Build the internal request struct using the user's options.
    let body = DnsRecordEditRequest {
      auth: self.client.auth.clone(),
      name: options.name,
      r#type: options.r#type,
      content: options.content,
      ttl: options.ttl,
      prio: options.prio,
    };
    self.client.post(&path, &body).await
  }

  /// Deletes a specific DNS record by its ID.
  ///
  /// # Arguments
  /// * `record_id` - The numeric ID of the record to delete.
  pub async fn delete_record_by_id(&self, record_id: u64) -> Result<StatusResponse> {
    let path = format!("{}{}/{}", endpoints::DNS_DELETE_BY_ID, self.domain, record_id);
    self.client.post(&path, &self.client.auth).await
  }

  /// Retrieves all DNS records for the domain.
  pub async fn retrieve_all_records(&self) -> Result<Vec<DnsRecord>> {
    let path = format!("{}{}", endpoints::DNS_RETRIEVE_BY_DOMAIN, self.domain);
    let response: DnsRecordListResponse = self.client.post(&path, &self.client.auth).await?;
    Ok(response.records)
  }

  /// Retrieves a single DNS record by its ID.
  ///
  /// Returns `Ok(None)` if no record with the specified ID is found.
  ///
  /// # Arguments
  /// * `record_id` - The numeric ID of the record to retrieve.
  pub async fn retrieve_record_by_id(&self, record_id: u64) -> Result<Option<DnsRecord>> {
    let path = format!("{}{}/{}", endpoints::DNS_RETRIEVE_BY_DOMAIN, self.domain, record_id);
    let response: DnsRecordListResponse = self.client.post(&path, &self.client.auth).await?;
    Ok(response.records.into_iter().next())
  }

  /// Retrieves all records that match a given name and type.
  ///
  /// # Arguments
  /// * `record_type` - The type of records to retrieve (e.g., "A", "CNAME").
  /// * `subdomain` - The subdomain to match. Use an empty string for the root domain.
  pub async fn retrieve_records_by_name_type(&self, record_type: &str, subdomain: &str) -> Result<Vec<DnsRecord>> {
    let mut path = format!(
      "{}{}/{}",
      endpoints::DNS_RETRIEVE_BY_NAME_TYPE,
      self.domain,
      record_type
    );
    if !subdomain.is_empty() {
      path.push('/');
      path.push_str(subdomain);
    }
    let response: DnsRecordListResponse = self.client.post(&path, &self.client.auth).await?;
    Ok(response.records)
  }

  // --- DNSSEC Methods ---

  /// Creates a new DNSSEC record at the registry.
  ///
  /// # Arguments
  /// * `record` - A reference to a `DnssecRecord` struct with the new record's data.
  pub async fn create_dnssec_record(&self, record: &'a DnssecRecord) -> Result<StatusResponse> {
    let path = format!("{}{}", endpoints::DNSSEC_CREATE, self.domain);
    let body = DnssecCreateRequest {
      auth: self.client.auth.clone(),
      record,
    };
    self.client.post(&path, &body).await
  }

  /// Retrieves all DNSSEC records for the domain from the registry.
  pub async fn get_dnssec_records(&self) -> Result<std::collections::HashMap<String, DnssecRecord>> {
    let path = format!("{}{}", endpoints::DNSSEC_GET, self.domain);
    let response: DnssecRecordListResponse = self.client.post(&path, &self.client.auth).await?;
    Ok(response.records)
  }

  /// Deletes a DNSSEC record from the registry by its key tag.
  ///
  /// # Arguments
  /// * `key_tag` - The key tag of the record to delete.
  pub async fn delete_dnssec_record(&self, key_tag: &str) -> Result<StatusResponse> {
    let path = format!("{}{}/{}", endpoints::DNSSEC_DELETE, self.domain, key_tag);
    self.client.post(&path, &self.client.auth).await
  }
}
