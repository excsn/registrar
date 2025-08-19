//! The primary Name.com client and its core methods, including authentication and HTTP helpers.

use super::{
  dns::DnsClient,
  domain::DomainsClient,
  endpoints,
  types::{ErrorResponse, Hello},
  url_forwarding::UrlForwardingClient,
  vanity_ns::VanityNameserverClient,
};
use crate::{Error, Result};
use reqwest::{Client as HttpClient, Response, StatusCode};
use serde::{Serialize, de::DeserializeOwned};

/// The primary client for interacting with the Name.com Core V1 API.
///
/// It holds the authentication credentials and an HTTP client, and provides
/// access to the various API functional groups via sub-clients.
#[derive(Clone, Debug)]
pub struct NameDotCom {
  http_client: HttpClient,
  host: String,
  username: String,
  token: String,
}

impl NameDotCom {
  /// The production API host.
  pub const PRODUCTION_HOST: &'static str = "https://api.name.com";
  /// The development/testing API host.
  pub const DEVELOPMENT_HOST: &'static str = "https://api.dev.name.com";

  /// Creates a new Name.com client for the production environment.
  pub fn new(username: String, token: String) -> Self {
    Self::with_host(Self::PRODUCTION_HOST.to_string(), username, token)
  }

  pub fn new_dev(username: String, token: String) -> Self {
    Self::with_host(Self::DEVELOPMENT_HOST.to_string(), username, token)
  }

  /// Creates a new Name.com client for a custom environment (e.g., development).
  pub fn with_host(host: String, username: String, token: String) -> Self {
    Self {
      host,
      username,
      token,
      http_client: HttpClient::new(),
    }
  }

  /// A simple endpoint to test connectivity to the Name.com API server.
  pub async fn hello(&self) -> Result<Hello> {
    let url = format!("{}{}", self.host, endpoints::HELLO);
    // Add the required authentication call
    let response = self
      .http_client
      .get(&url)
      .basic_auth(&self.username, Some(&self.token))
      .send()
      .await?;
    // Use the handler designated for responses with bodies.
    Self::handle_response_with_body(response).await
  }

  // --- Sub-Client Constructors ---\
  pub fn domains<'a>(&'a self) -> DomainsClient<'a> {
    DomainsClient::new(self)
  }

  pub fn dns<'a>(&'a self, domain_name: &'a str) -> DnsClient<'a> {
    DnsClient::new(self, domain_name)
  }
  
  pub fn url_forwarding<'a>(&'a self, domain_name: &'a str) -> UrlForwardingClient<'a> {
    UrlForwardingClient::new(self, domain_name)
  }

  pub fn vanity_ns<'a>(&'a self, domain_name: &'a str) -> VanityNameserverClient<'a> {
    VanityNameserverClient::new(self, domain_name)
  }

  // --- Internal HTTP Helpers ---

  pub(super) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
    let url = format!("{}{}", self.host, path);
    let response = self
      .http_client
      .get(&url)
      .basic_auth(&self.username, Some(&self.token))
      .send()
      .await?;
    Self::handle_response_with_body(response).await
  }

  pub(super) async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: B) -> Result<T> {
    let url = format!("{}{}", self.host, path);
    let response = self
      .http_client
      .post(&url)
      .basic_auth(&self.username, Some(&self.token))
      .json(&body)
      .send()
      .await?;
    Self::handle_response_with_body(response).await
  }

  pub(super) async fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: B) -> Result<T> {
    let url = format!("{}{}", self.host, path);
    let response = self
      .http_client
      .put(&url)
      .basic_auth(&self.username, Some(&self.token))
      .json(&body)
      .send()
      .await?;
    Self::handle_response_with_body(response).await
  }

  pub(super) async fn patch<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: B) -> Result<T> {
    let url = format!("{}{}", self.host, path);
    let response = self
      .http_client
      .patch(&url)
      .basic_auth(&self.username, Some(&self.token))
      .json(&body)
      .send()
      .await?;
    Self::handle_response_with_body(response).await
  }

  pub(super) async fn delete(&self, path: &str) -> Result<()> {
    let url = format!("{}{}", self.host, path);
    let response = self
      .http_client
      .delete(&url)
      .basic_auth(&self.username, Some(&self.token))
      .send()
      .await?;
    Self::handle_empty_response(response).await
  }

  // --- Private Response Handlers ---

  /// A centralized function to handle API responses that are expected to have a JSON body.
  async fn handle_response_with_body<T: DeserializeOwned>(response: Response) -> Result<T> {
    match response.status() {
      StatusCode::OK | StatusCode::CREATED => response.json().await.map_err(Error::Http),
      _ => Err(Self::build_api_error(response).await),
    }
  }

  /// A centralized function to handle API responses that are successful with no body (204).
  async fn handle_empty_response(response: Response) -> Result<()> {
    match response.status() {
      StatusCode::NO_CONTENT => Ok(()),
      _ => Err(Self::build_api_error(response).await),
    }
  }

  /// Helper to build an `Error::Api` from an error response body.
  async fn build_api_error(response: Response) -> Error {
    let error_text = response.text().await.unwrap_or_else(|e| e.to_string());
    if let Ok(api_error) = serde_json::from_str::<ErrorResponse>(&error_text) {
      Error::Api(api_error.message)
    } else {
      Error::Api(error_text)
    }
  }
}
