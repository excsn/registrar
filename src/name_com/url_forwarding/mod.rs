//! The UrlForwarding sub-client and its methods for the Name.com Core API.

use self::types::{
  ListUrlForwardingResponse, UrlForwardingCreatePayload, UrlForwardingRecord, UrlForwardingUpdatePayload,
};
use super::{client::NameDotCom, endpoints};
use crate::Result;

// Re-export the public types for this module.
pub mod types;

/// Provides access to URL Forwarding functionality for a specific domain.
///
/// Created via `NameCom::url_forwarding("example.org")`.
pub struct UrlForwardingClient<'a> {
  client: &'a NameDotCom,
  domain_name: &'a str,
}

impl<'a> UrlForwardingClient<'a> {
  /// Constructor is internal to the `name_com` module.
  pub(super) fn new(client: &'a NameDotCom, domain_name: &'a str) -> Self {
    Self { client, domain_name }
  }

  /// Retrieves a list of all URL forwarding records for the domain.
  /// This method handles pagination internally.
  pub async fn list(&self) -> Result<Vec<UrlForwardingRecord>> {
    let mut all_records = Vec::new();
    let mut page = 1;
    loop {
      let path = format!(
        "{}{}{}?page={}",
        endpoints::CORE_V1_DOMAINS_PREFIX,
        self.domain_name,
        endpoints::CORE_V1_URL_FORWARDING_SUFFIX,
        page
      );
      let response: ListUrlForwardingResponse = self.client.get(&path).await?;

      all_records.extend(response.forwards);

      if response.next_page.is_none() {
        break;
      }
      page = response.next_page.unwrap();
    }
    Ok(all_records)
  }

  /// Retrieves a single URL forwarding record by its host.
  ///
  /// # Arguments
  /// * `host` - The full hostname for the forwarding rule (e.g., "www.example.org").
  pub async fn get(&self, host: &str) -> Result<UrlForwardingRecord> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_URL_FORWARDING_SUFFIX,
      host
    );
    self.client.get(&path).await
  }

  /// Creates a new URL forwarding record.
  pub async fn create(&self, payload: UrlForwardingCreatePayload<'_>) -> Result<UrlForwardingRecord> {
    let path = format!(
      "{}{}{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_URL_FORWARDING_SUFFIX
    );
    self.client.post(&path, payload).await
  }

  /// Updates an existing URL forwarding record.
  ///
  /// # Arguments
  /// * `host` - The full hostname of the rule to update (e.g., "www.example.org").
  /// * `payload` - A struct with the new forwarding details.
  pub async fn update(&self, host: &str, payload: UrlForwardingUpdatePayload<'_>) -> Result<UrlForwardingRecord> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_URL_FORWARDING_SUFFIX,
      host
    );
    self.client.put(&path, payload).await
  }

  /// Deletes a URL forwarding record by its host.
  ///
  /// # Arguments
  /// * `host` - The full hostname of the rule to delete (e.g., "www.example.org").
  pub async fn delete(&self, host: &str) -> Result<()> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_URL_FORWARDING_SUFFIX,
      host
    );
    self.client.delete(&path).await
  }
}
