//! The Ssl sub-client and its methods.

use self::types::SslBundleResponse;
use super::{client::Porkbun, endpoints};
use crate::Result;

// Re-export the public types for this module to be used in `porkbun/mod.rs`
pub mod types;

/// Provides access to the SSL functionality of the Porkbun API.
///
/// Created via `Porkbun::ssl("example.com")`.
pub struct Ssl<'a> {
  client: &'a Porkbun,
  domain: &'a str,
}

impl<'a> Ssl<'a> {
  // Constructor is internal to the `porkbun` module.
  pub(super) fn new(client: &'a Porkbun, domain: &'a str) -> Self {
    Self { client, domain }
  }

  /// Retrieves the SSL certificate bundle for the specified domain.
  ///
  /// The bundle includes the private key, the full certificate chain,
  /// and the public key.
  pub async fn retrieve_bundle(&self) -> Result<SslBundleResponse> {
    let path = format!("{}{}", endpoints::SSL_RETRIEVE_BUNDLE, self.domain);
    self.client.post(&path, &self.client.auth).await
  }
}
