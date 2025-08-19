// src/name_com/domain/mod.rs

//! The Domain sub-client and its methods for the Name.com Core API.

use self::types::{
  AvailabilityResult, CheckAvailabilityRequest, CheckAvailabilityResponse, CreateDomainRequest, CreateDomainResponse,
  Domain, DomainCreatePayload, GetAuthCodeResponse, ListDomainsResponse, SetNameserversRequest, UpdateDomainPayload,
};
use super::{client::NameDotCom, endpoints};
use crate::Result;

// Re-export the public types for this module.
pub mod types;

/// Provides access to domain-related functionality not specific to one domain.
pub struct DomainsClient<'a> {
  client: &'a NameDotCom,
}

impl<'a> DomainsClient<'a> {
  pub(super) fn new(client: &'a NameDotCom) -> Self {
    Self { client }
  }

  /// Retrieves a list of all domains in your account.
  pub async fn list(&self) -> Result<Vec<Domain>> {
    let mut all_domains = Vec::new();
    let mut page = 1;
    loop {
      let path = format!("{}?page={}", endpoints::CORE_V1_DOMAINS_PREFIX, page);
      let response: ListDomainsResponse = self.client.get(&path).await?;

      all_domains.extend(response.domains);

      if response.next_page.is_none() {
        break;
      }
      page = response.next_page.unwrap();
    }
    Ok(all_domains)
  }

  /// Checks the availability of a list of domain names.
  pub async fn check_availability(&self, domain_names: &[&str]) -> Result<Vec<AvailabilityResult>> {
    let path = format!(
      "{}{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      endpoints::CORE_V1_ACTION_CHECK_AVAILABILITY
    );
    let body = CheckAvailabilityRequest {
      domain_names: domain_names.iter().map(|s| s.to_string()).collect(),
    };
    let response: CheckAvailabilityResponse = self.client.post(&path, body).await?;
    Ok(response.results)
  }

  /// Registers a new domain.
  /// NOTE: In the dev environment, this only simulates the registration.
  pub async fn create(&self, domain_name: &str) -> Result<CreateDomainResponse> {
    let payload = DomainCreatePayload { domain_name };
    let body = CreateDomainRequest { domain: payload };
    self.client.post(endpoints::CORE_V1_DOMAINS_PREFIX, body).await
  }

  /// Returns a client for operating on a single, specific domain.
  pub fn domain(&self, domain_name: &'a str) -> DomainClient<'a> {
    DomainClient {
      client: self.client,
      domain_name,
    }
  }
}

/// Provides access to functionality for a specific domain.
pub struct DomainClient<'a> {
  client: &'a NameDotCom,
  domain_name: &'a str,
}

impl<'a> DomainClient<'a> {
  pub(super) fn new(client: &'a NameDotCom, domain_name: &'a str) -> Self {
    Self { client, domain_name }
  }

  /// Retrieves the details for this specific domain.
  pub async fn get(&self) -> Result<Domain> {
    let path = format!("{}{}", endpoints::CORE_V1_DOMAINS_PREFIX, self.domain_name);
    self.client.get(&path).await
  }

  /// Updates the autorenew, privacy, or lock status for the domain.
  pub async fn update(&self, payload: UpdateDomainPayload) -> Result<Domain> {
    let path = format!("{}{}", endpoints::CORE_V1_DOMAINS_PREFIX, self.domain_name);
    self.client.patch(&path, payload).await
  }

  /// Retrieves the transfer authorization code (EPP code) for a domain.
  pub async fn get_auth_code(&self) -> Result<String> {
    let path = format!(
      "{}{}{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_ACTION_GET_AUTH_CODE
    );
    let response: GetAuthCodeResponse = self.client.get(&path).await?;
    Ok(response.auth_code)
  }

  /// Sets the nameservers for the domain.
  pub async fn set_nameservers(&self, nameservers: &[&str]) -> Result<Domain> {
    let path = format!(
      "{}{}{}",
      endpoints::CORE_V1_DOMAINS_PREFIX,
      self.domain_name,
      endpoints::CORE_V1_ACTION_SET_NAMESERVERS
    );
    let body = SetNameserversRequest {
      nameservers: nameservers.iter().map(|s| s.to_string()).collect(),
    };
    self.client.post(&path, body).await
  }
}
