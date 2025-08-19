//! The VanityNs sub-client and its methods for the Name.com Core API.

use super::{client::NameDotCom, endpoints};
use crate::Result;
use self::types::{
  ListVanityNsResponse, VanityNameserver, VanityNsCreatePayload, VanityNsUpdatePayload,
};

// Re-export the public types for this module.
pub mod types;

/// Provides access to Vanity Nameserver functionality for a specific domain.
///
/// Created via `NameCom::vanity_ns("example.org")`.
pub struct VanityNameserverClient<'a> {
  client: &'a NameDotCom,
  domain_name: &'a str,
}

impl<'a> VanityNameserverClient<'a> {
  /// Constructor is internal to the `name_com` module.
  pub(super) fn new(client: &'a NameDotCom, domain_name: &'a str) -> Self {
    Self { client, domain_name }
  }

  /// Retrieves a list of all vanity nameservers for the domain.
  /// This method handles pagination internally.
  pub async fn list(&self) -> Result<Vec<VanityNameserver>> {
    let mut all_records = Vec::new();
    let mut page = 1;
    loop {
      let path = format!(
        "{}{}{}?page={}",
        endpoints::CORE_V1_DOMAINS_PREFIX,
        self.domain_name,
        endpoints::CORE_V1_VANITY_NS_SUFFIX,
        page
      );
      let response: ListVanityNsResponse = self.client.get(&path).await?;

      all_records.extend(response.vanity_ns);

      if response.next_page.is_none() {
        break;
      }
      page = response.next_page.unwrap();
    }
    Ok(all_records)
  }

  /// Retrieves a single vanity nameserver by its hostname.
  ///
  /// # Arguments
  /// * `hostname` - The hostname of the vanity nameserver (e.g., "ns1.example.org").
  pub async fn get(&self, hostname: &str) -> Result<VanityNameserver> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_VANITY_NS_SUFFIX,
      hostname
    );
    self.client.get(&path).await
  }

  /// Creates a new vanity nameserver.
  pub async fn create(&self, payload: VanityNsCreatePayload<'_>) -> Result<VanityNameserver> {
    let path = format!(
      "{}{}{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_VANITY_NS_SUFFIX
    );
    self.client.post(&path, payload).await
  }

  /// Updates an existing vanity nameserver.
  ///
  /// # Arguments
  /// * `hostname` - The hostname of the vanity nameserver to update.
  /// * `payload` - A struct with the new IP addresses.
  pub async fn update(
    &self,
    hostname: &str,
    payload: VanityNsUpdatePayload<'_>,
  ) -> Result<VanityNameserver> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_VANITY_NS_SUFFIX,
      hostname
    );
    self.client.put(&path, payload).await
  }

  /// Deletes a vanity nameserver by its hostname.
  ///
  /// # Arguments
  /// * `hostname` - The hostname of the vanity nameserver to delete.
  pub async fn delete(&self, hostname: &str) -> Result<()> {
    let path = format!(
      "{}{}{}/{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_VANITY_NS_SUFFIX,
      hostname
    );
    self.client.delete(&path).await
  }
}