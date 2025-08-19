//! Contains top-level and shared data structures for the Name.com API.

use serde::Deserialize;

/// A standard error response from the Name.com API.
///
/// This is used internally to parse error messages when API calls fail,
/// and the message is then wrapped in our crate's `Error::Api`.
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ErrorResponse {
  pub message: String,
  #[serde(default)]
  pub details: String,
}

/// The response for a successful "hello" command.
/// This confirms connectivity to the Name.com API service.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hello {
  #[serde(default)] // Good practice in case they ever omit it
  pub motd: String,
  pub server_name: String,
  pub server_time: String,
  pub username: String,
}
