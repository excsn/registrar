//! The Dns sub-client and its methods for the Name.com API.

use self::types::{
  DnsRecord, DnsRecordPayload, DnssecCreatePayload, DnssecRecord, ListDnsRecordsResponse, ListDnssecResponse,
};
use super::{client::NameDotCom, endpoints};
use crate::Result;

// Re-export the public types for this module.
pub mod types;

/// Provides access to DNS and DNSSEC functionality for a specific domain.
///
/// Created via `NameCom::dns("example.org")`.
pub struct DnsClient<'a> {
  client: &'a NameDotCom,
  domain_name: &'a str,
}

impl<'a> DnsClient<'a> {
  /// Constructor is internal to the `name_com` module.
  pub(super) fn new(client: &'a NameDotCom, domain_name: &'a str) -> Self {
    Self { client, domain_name }
  }

  // --- Standard DNS Record Methods ---

  /// Retrieves a list of all DNS records for the domain.
  /// This method handles pagination internally.
  pub async fn list_records(&self) -> Result<Vec<DnsRecord>> {
    let mut all_records = Vec::new();
    let mut page = 1;
    loop {
      let path = format!(
        "{}{}{}?page={}",
        endpoints::CORE_V1_DOMAINS_PREFIX,
        self.domain_name,
        endpoints::CORE_V1_RECORDS_SUFFIX,
        page
      );
      let response: ListDnsRecordsResponse = self.client.get(&path).await?;

      all_records.extend(response.records);

      if response.next_page.is_none() {
        break;
      }
      page = response.next_page.unwrap();
    }
    Ok(all_records)
  }

  /// Retrieves a single DNS record by its ID.
  pub async fn get_record(&self, record_id: i32) -> Result<DnsRecord> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_RECORDS_SUFFIX,
      record_id
    );
    self.client.get(&path).await
  }

  /// Creates a new DNS record.
  pub async fn create_record(&self, payload: DnsRecordPayload<'_>) -> Result<DnsRecord> {
    let path = format!(
      "{}{}{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_RECORDS_SUFFIX,
    );
    self.client.post(&path, payload).await
  }

  /// Updates an existing DNS record.
  pub async fn update_record(&self, record_id: i32, payload: DnsRecordPayload<'_>) -> Result<DnsRecord> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_RECORDS_SUFFIX,
      record_id
    );
    self.client.put(&path, payload).await
  }

  /// Deletes a DNS record by its ID.
  pub async fn delete_record(&self, record_id: i32) -> Result<()> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_RECORDS_SUFFIX,
      record_id
    );
    self.client.delete(&path).await
  }

  // --- DNSSEC Methods ---

  /// Retrieves a list of all DNSSEC records for the domain.
  pub async fn list_dnssec(&self) -> Result<Vec<DnssecRecord>> {
    let path = format!(
      "{}{}{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_DNSSEC_SUFFIX
    );
    let response: ListDnssecResponse = self.client.get(&path).await?;
    Ok(response.dnssec)
  }

  /// Retrieves a single DNSSEC record by its digest.
  pub async fn get_dnssec(&self, digest: &str) -> Result<DnssecRecord> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_DNSSEC_SUFFIX,
      digest
    );
    self.client.get(&path).await
  }

  /// Creates a new DNSSEC record for the domain.
  pub async fn create_dnssec(&self, payload: DnssecCreatePayload<'_>) -> Result<DnssecRecord> {
    let path = format!(
      "{}{}{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_DNSSEC_SUFFIX
    );
    self.client.post(&path, payload).await
  }

  /// Deletes a DNSSEC record from the domain by its digest.
  pub async fn delete_dnssec(&self, digest: &str) -> Result<()> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_DNSSEC_SUFFIX,
      digest
    );
    self.client.delete(&path).await
  }
}
