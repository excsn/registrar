//! The primary Porkbun client and its core methods.

use super::{
  dns::Dns,
  domain::Domain,
  endpoints,
  ssl::Ssl,
  types::{Auth, PingResponse, PricingResponse, StatusResponse},
};
use crate::{Error, Result};
use reqwest::Client as HttpClient;
use serde::{Serialize, de::DeserializeOwned};

/// The primary client for interacting with the Porkbun v3 API.
///
/// It holds the authentication credentials and an HTTP client, and provides
/// access to the various API functional groups via sub-clients.
#[derive(Clone, Debug)]
pub struct Porkbun {
  // A reqwest client, which is cheap to clone and manages connection pooling.
  http_client: HttpClient,
  // Authentication details, cloned into each request body.
  pub(super) auth: Auth,
}

impl Porkbun {
  /// Creates a new Porkbun client with the provided API and Secret keys.
  ///
  /// # Arguments
  /// * `apikey` - Your Porkbun API key.
  /// * `secretapikey` - Your Porkbun Secret API key.
  pub fn new(apikey: String, secretapikey: String) -> Self {
    Self {
      http_client: HttpClient::new(),
      auth: Auth { apikey, secretapikey },
    }
  }

  /// Pings the Porkbun API to test credentials and returns your public IP address.
  ///
  /// A successful response (`Ok(...)`) confirms that your credentials are correct.
  pub async fn ping(&self) -> Result<PingResponse> {
    self.post(endpoints::PING, &self.auth).await
  }

  /// Retrieves the pricing for all TLDs.
  ///
  /// This endpoint does not require authentication.
  pub async fn get_pricing(&self) -> Result<PricingResponse> {
    self.post_unauthenticated(endpoints::PRICING_GET).await
  }

  // --- Sub-Client Constructors ---

  /// Access domain-specific functionality.
  ///
  /// # Arguments
  /// * `domain` - The domain name to operate on (e.g., "example.com").
  pub fn domain<'a>(&'a self, domain: &'a str) -> Domain<'a> {
    Domain::new(self, domain)
  }

  /// Access DNS-specific functionality.
  ///
  /// # Arguments
  /// * `domain` - The domain name whose DNS records you want to manage.
  pub fn dns<'a>(&'a self, domain: &'a str) -> Dns<'a> {
    Dns::new(self, domain)
  }

  /// Access SSL-specific functionality.
  ///
  /// # Arguments
  /// * `domain` - The domain name whose SSL bundle you want to retrieve.
  pub fn ssl<'a>(&'a self, domain: &'a str) -> Ssl<'a> {
    Ssl::new(self, domain)
  }

  // --- Internal HTTP Helpers ---

  /// A generic helper for making authenticated POST requests.
  ///
  /// It serializes the provided body, handles response status checking,
  /// and deserializes the JSON into the target type `T`.
  pub(super) async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
  where
    T: DeserializeOwned,
    B: Serialize,
  {
    let url = format!("{}{}", endpoints::BASE_URL, path);

    let response_text = self
      .http_client
      .post(&url)
      .json(body)
      .send()
      .await?
      .error_for_status()? // Ensure we have a 2xx status code
      .text()
      .await?;

    // First, check for an API-level error status.
    let status_check: StatusResponse = serde_json::from_str(&response_text)?;
    if status_check.status == "ERROR" {
      return Err(Error::Api(
        status_check.message.unwrap_or_else(|| "Unknown API error".to_string()),
      ));
    }

    // If the status is not ERROR, deserialize to the final target type.
    let final_response: T = serde_json::from_str(&response_text)?;
    Ok(final_response)
  }

  /// A helper for unauthenticated POST requests, like the pricing endpoint.
  /// It sends an empty JSON object `{}` as the body.
  async fn post_unauthenticated<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
    let url = format!("{}{}", endpoints::BASE_URL, path);

    let response_text = self
      .http_client
      .post(&url)
      .json(&serde_json::json!({})) // Send an empty JSON object
      .send()
      .await?
      .error_for_status()?
      .text()
      .await?;

    let status_check: StatusResponse = serde_json::from_str(&response_text)?;
    if status_check.status == "ERROR" {
      return Err(Error::Api(
        status_check.message.unwrap_or_else(|| "Unknown API error".to_string()),
      ));
    }

    let final_response: T = serde_json::from_str(&response_text)?;
    Ok(final_response)
  }
}
