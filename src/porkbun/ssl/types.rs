//! Contains all serde structs for the Porkbun SSL API endpoints.

use serde::Deserialize;

/// The response containing the SSL certificate bundle for a domain.
///
/// Each field contains the respective part of the bundle in PEM format.
#[derive(Deserialize, Debug, Clone)]
pub struct SslBundleResponse {
  pub status: String,
  // Field names match the API response exactly.
  pub certificatechain: String,
  pub privatekey: String,
  pub publickey: String,
}