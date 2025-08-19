# **Registrar API Reference**

This document provides a complete and detailed reference for all public API components of the `registrar` crate. It is intended to be an exhaustive resource for developers, reflecting the exact public interface of the library.

---

### **1. Universal API Components**

These components are available at the crate root (`registrar::*`) and are used by all provider implementations.

#### **1.1. Error Handling**

The crate uses a unified error type for all operations.

*   **Type Alias: `registrar::Result<T>`**
    *   The standard `Result` type used for all fallible operations in this crate, aliasing `std::result::Result<T, registrar::Error>`. It simplifies function signatures throughout the crate.
    ```rust
    pub type Result<T> = std::result::Result<T, Error>;
    ```

*   **Enum: `registrar::Error`**
    *   The universal error enum for the crate. This enum consolidates errors from the underlying HTTP client, JSON processing, and specific API error messages.
    ```rust
    #[derive(Error, Debug)]
    pub enum Error {
      /// An error occurred during the HTTP request. This could be a network issue,
      /// a DNS problem, or an invalid certificate. It contains the original error
      /// from the `reqwest` library.
      #[error("HTTP request failed: {0}")]
      Http(#[from] reqwest::Error),

      /// An error occurred while serializing a request to JSON or deserializing
      /// a response from JSON. It contains the original error from the `serde_json`
      /// library.
      #[error("Failed to parse JSON: {0}")]
      Json(#[from] serde_json::Error),

      /// The registrar's API returned a specific error message (e.g., "Invalid API Key").
      /// This typically corresponds to a logical error, even with a successful HTTP status.
      /// It contains the descriptive error message string from the API.
      #[error("API Error: {0}")]
      Api(String),
    }
    ```

---

### **2. Porkbun API Client (`registrar::porkbun`)**

*Requires the `"porkbun"` feature to be enabled in `Cargo.toml`.*

The Porkbun module provides a client and all necessary types for interacting with the Porkbun v3 API.

#### **2.1. Main Client: `porkbun::Porkbun`**

The primary client and entry point for all interactions with the Porkbun v3 API. It holds authentication credentials and an HTTP client.

##### **Constructors**

*   **`pub fn new(apikey: String, secretapikey: String) -> Self`**
    *   Creates a new Porkbun client for the production API.
    *   **Parameters:**
        *   `apikey: String`: Your Porkbun API key.
        *   `secretapikey: String`: Your Porkbun Secret API key.

##### **Methods**

*   **`pub async fn ping(&self) -> Result<PingResponse>`**
    *   Pings the Porkbun API to test credentials. A successful `Ok(...)` response confirms that your authentication details are correct. The response includes your public IP address as seen by the Porkbun servers.

*   **`pub async fn get_pricing(&self) -> Result<PricingResponse>`**
    *   Retrieves the pricing for all supported Top-Level Domains (TLDs). This endpoint does not require authentication.

##### **Sub-Client Accessors**

*   **`pub fn domain<'a>(&'a self, domain: &'a str) -> domain::Domain<'a>`**
    *   Returns a client for performing domain-specific actions like managing nameservers, URL forwarding, and glue records.
    *   **Parameters:**
        *   `domain: &'a str`: The domain name to operate on (e.g., "example.com").

*   **`pub fn dns<'a>(&'a self, domain: &'a str) -> dns::Dns<'a>`**
    *   Returns a client for performing DNS and DNSSEC record management.
    *   **Parameters:**
        *   `domain: &'a str`: The domain name whose DNS records you want to manage.

*   **`pub fn ssl<'a>(&'a self, domain: &'a str) -> ssl::Ssl<'a>`**
    *   Returns a client for SSL-related actions, specifically for retrieving certificate bundles.
    *   **Parameters:**
        *   `domain: &'a str`: The domain name whose SSL bundle you want to retrieve.

#### **2.2. Sub-Clients**

##### **`porkbun::domain::Domain<'a>`**
Provides methods for domain-specific functionality. Instantiated by calling `Porkbun::domain(...)`.

*   **`pub async fn update_nameservers(&self, nameservers: &[&str]) -> Result<StatusResponse>`**
    *   Updates the authoritative nameservers for the domain.
*   **`pub async fn get_nameservers(&self) -> Result<NameserverListResponse>`**
    *   Retrieves the current authoritative nameservers for the domain.
*   **`pub async fn list_all(&self, include_labels: bool) -> Result<Vec<DomainInfo>>`**
    *   Retrieves all domains in your account. This method handles pagination internally to return a complete list.
    *   **Parameters:**
        *   `include_labels: bool`: If `true`, includes any labels assigned to domains in the response.
*   **`pub async fn add_url_forward(&self, options: &UrlForwardRecord) -> Result<StatusResponse>`**
    *   Adds a URL forwarding record for the domain.
*   **`pub async fn get_url_forwarding(&self) -> Result<Vec<UrlForwardRecord>>`**
    *   Retrieves all URL forwarding records for the domain.
*   **`pub async fn delete_url_forward(&self, record_id: u64) -> Result<StatusResponse>`**
    *   Deletes a specific URL forwarding record by its numeric ID.
*   **`pub async fn check(&self) -> Result<DomainCheckResponse>`**
    *   Checks the availability of the domain.
*   **`pub async fn create_glue_record(&self, subdomain: &str, ips: &[std::net::IpAddr]) -> Result<StatusResponse>`**
    *   Creates a glue record for a subdomain of the current domain.
*   **`pub async fn update_glue_record(&self, subdomain: &str, ips: &[std::net::IpAddr]) -> Result<StatusResponse>`**
    *   Updates an existing glue record, replacing its IP addresses.
*   **`pub async fn delete_glue_record(&self, subdomain: &str) -> Result<StatusResponse>`**
    *   Deletes a glue record.
*   **`pub async fn get_glue_records(&self) -> Result<Vec<(String, GlueRecordIps)>>`**
    *   Retrieves all glue records for the domain.

##### **`porkbun::dns::Dns<'a>`**
Provides methods for managing DNS records. Instantiated by calling `Porkbun::dns(...)`.

*   **`pub async fn create_record(&self, options: DnsRecordCreateOptions<'_>) -> Result<DnsRecordCreateResponse>`**
    *   Creates a new DNS record. The details are specified in the `options` payload.
*   **`pub async fn edit_record_by_id(&self, record_id: u64, options: DnsRecordEditOptions<'_>) -> Result<StatusResponse>`**
    *   Edits a specific DNS record by its ID. Only the fields set in the `options` payload will be changed.
*   **`pub async fn delete_record_by_id(&self, record_id: u64) -> Result<StatusResponse>`**
    *   Deletes a specific DNS record by its numeric ID.
*   **`pub async fn retrieve_all_records(&self) -> Result<Vec<DnsRecord>>`**
    *   Retrieves all DNS records for the domain.
*   **`pub async fn retrieve_record_by_id(&self, record_id: u64) -> Result<Option<DnsRecord>>`**
    *   Retrieves a single DNS record by its ID. Returns `Ok(None)` if no record with that ID is found.
*   **`pub async fn retrieve_records_by_name_type(&self, record_type: &str, subdomain: &str) -> Result<Vec<DnsRecord>>`**
    *   Retrieves all records that match a given name and type. Use an empty string for `subdomain` to match the root domain.
*   **`pub async fn create_dnssec_record(&self, record: &DnssecRecord) -> Result<StatusResponse>`**
    *   Creates a new DNSSEC record at the registry.
*   **`pub async fn get_dnssec_records(&self) -> Result<std::collections::HashMap<String, DnssecRecord>>`**
    *   Retrieves all DNSSEC records for the domain from the registry.
*   **`pub async fn delete_dnssec_record(&self, key_tag: &str) -> Result<StatusResponse>`**
    *   Deletes a DNSSEC record from the registry by its key tag.

##### **`porkbun::ssl::Ssl<'a>`**
Provides methods for SSL actions. Instantiated by calling `Porkbun::ssl(...)`.

*   **`pub async fn retrieve_bundle(&self) -> Result<SslBundleResponse>`**
    *   Retrieves the SSL certificate bundle for the specified domain, including the private key, full certificate chain, and public key.

#### **2.3. Public Data Structures**
This section lists all public request and response structs for the Porkbun client. They are found across the various `types.rs` files within the `porkbun` module.

*   **`struct StatusResponse`**: A generic response indicating the status of an operation.
    *   `pub status: String`
    *   `pub message: Option<String>`
*   **`struct PingResponse`**: The response for a successful ping request.
    *   `pub status: String`
    *   `pub your_ip: String`
*   **`struct TldPricing`**: Pricing information for a single Top-Level Domain.
    *   `pub registration: String`
    *   `pub renewal: String`
    *   `pub transfer: String`
*   **`struct PricingResponse`**: Contains pricing for all supported TLDs.
    *   `pub status: String`
    *   `pub pricing: std::collections::HashMap<String, TldPricing>`
*   **`struct NameserverListResponse`**: Response containing a list of nameservers.
    *   `pub status: String`
    *   `pub ns: Vec<String>`
*   **`struct Label`**: A label associated with a domain in your account.
    *   `pub id: String`
    *   `pub title: String`
    *   `pub color: String`
*   **`struct DomainInfo`**: Detailed information about a single domain.
    *   `pub domain: String`
    *   `pub status: String`
    *   `pub tld: String`
    *   `pub create_date: String`
    *   `pub expire_date: String`
    *   `pub security_lock: String` ("1" or "0")
    *   `pub whois_privacy: String` ("1" or "0")
    *   `pub auto_renew: u8` (1 or 0)
    *   `pub not_local: u8` (1 or 0)
    *   `pub labels: Vec<Label>`
*   **`struct UrlForwardRecord`**: Represents a single URL forwarding record.
    *   `pub id: String`
    *   `pub subdomain: String`
    *   `pub location: String`
    *   `pub r#type: String`
    *   `pub include_path: String`
    *   `pub wildcard: String`
*   **`struct PriceInfo`**: Pricing details for a specific action (renewal, transfer).
    *   `pub r#type: String`
    *   `pub price: String`
    *   `pub regular_price: String`
*   **`struct AdditionalPricing`**: Contains pricing for additional domain actions.
    *   `pub renewal: PriceInfo`
    *   `pub transfer: PriceInfo`
*   **`struct DomainAvailability`**: Detailed availability information for a domain.
    *   `pub avail: String` ("yes" or "no")
    *   `pub r#type: String`
    *   `pub price: String`
    *   `pub first_year_promo: String` ("yes" or "no")
    *   `pub regular_price: String`
    *   `pub premium: String` ("yes" or "no")
    *   `pub additional: AdditionalPricing`
*   **`struct RateLimitInfo`**: Information about rate limits for domain checks.
    *   `pub ttl: String`
    *   `pub limit: String`
    *   `pub used: u64`
    *   `pub natural_language: String`
*   **`struct DomainCheckResponse`**: The full response for a domain availability check.
    *   `pub status: String`
    *   `pub response: DomainAvailability`
    *   `pub limits: RateLimitInfo`
*   **`struct GlueRecordIps`**: Represents the v4 and v6 IPs for a glue record host.
    *   `pub v6: Vec<std::net::IpAddr>`
    *   `pub v4: Vec<std::net::IpAddr>`
*   **`struct DnsRecord`**: Represents a single DNS record.
    *   `pub id: String`
    *   `pub name: String`
    *   `pub r#type: String`
    *   `pub content: String`
    *   `pub ttl: String`
    *   `pub prio: String`
    *   `pub notes: Option<String>`
*   **`struct DnsRecordCreateOptions<'a>`**: The public-facing options for creating a new DNS record.
    *   `pub name: Option<&'a str>`
    *   `pub r#type: &'a str`
    *   `pub content: &'a str`
    *   `pub ttl: Option<&'a str>`
    *   `pub prio: Option<&'a str>`
*   **`struct DnsRecordCreateResponse`**: Response after successfully creating a DNS record.
    *   `pub status: String`
    *   `pub id: u64`
*   **`struct DnsRecordEditOptions<'a>`**: Payload for editing a DNS record. All fields are optional.
    *   `pub name: Option<&'a str>`
    *   `pub r#type: Option<&'a str>`
    *   `pub content: Option<&'a str>`
    *   `pub ttl: Option<&'a str>`
    *   `pub prio: Option<&'a str>`
*   **`struct DnssecRecord`**: Represents a single DNSSEC record.
    *   `pub key_tag: String`
    *   `pub alg: String`
    *   `pub digest_type: String`
    *   `pub digest: String`
    *   `pub max_sig_life: Option<String>`
    *   `pub key_data_flags: Option<String>`
    *   `pub key_data_protocol: Option<String>`
    *   `pub key_data_algo: Option<String>`
    *   `pub key_data_pub_key: Option<String>`
*   **`struct SslBundleResponse`**: The response containing the SSL certificate bundle.
    *   `pub status: String`
    *   `pub certificatechain: String`
    *   `pub privatekey: String`
    *   `pub publickey: String`

---

### **3. Name.com API Client (`registrar::name_com`)**

*Requires the `"name-com"` feature to be enabled in `Cargo.toml`.*

The Name.com module provides a client and all necessary types for interacting with the **Name.com Core API**.

#### **3.1. Main Client: `name_com::NameDotCom`**

The primary client and entry point for all interactions with the **Name.com Core API**.

##### **Public Constants**

*   **`pub const PRODUCTION_HOST: &'static str = "https://api.name.com"`**
    *   The production API host.
*   **`pub const DEVELOPMENT_HOST: &'static str = "https://api.dev.name.com"`**
    *   The development/testing API host.

##### **Constructors**

*   **`pub fn new(username: String, token: String) -> Self`**
    *   Creates a new Name.com client for the **production** environment.
    *   **Parameters:**
        *   `username: String`: Your Name.com account username.
        *   `token: String`: Your Name.com API token.
*   **`pub fn new_dev(username: String, token: String) -> Self`**
    *   A convenience function that creates a new Name.com client for the **development** environment (`api.dev.name.com`).
    *   **Parameters:**
        *   `username: String`: Your Name.com account username.
        *   `token: String`: Your Name.com API token.
*   **`pub fn with_host(host: String, username: String, token: String) -> Self`**
    *   Creates a new Name.com client for a custom environment.
    *   **Parameters:**
        *   `host: String`: The base URL for the API server (e.g., `NameDotCom::PRODUCTION_HOST`).
        *   `username: String`: Your Name.com account username.
        *   `token: String`: Your Name.com API token.

##### **Methods**

*   **`pub async fn hello(&self) -> Result<Hello>`**
    *   A simple endpoint to test connectivity to the Name.com API server and confirm authentication is working.

##### **Sub-Client Accessors**

*   **`pub fn domains<'a>(&'a self) -> domain::DomainsClient<'a>`**
    *   Returns a client for performing general domain actions, such as listing all domains or checking availability.
*   **`pub fn dns<'a>(&'a self, domain_name: &'a str) -> dns::DnsClient<'a>`**
    *   Returns a client for managing DNS and DNSSEC records for a specific domain.
    *   **Parameters:**
        *   `domain_name: &'a str`: The domain name to operate on (e.g., "example.org").
*   **`pub fn url_forwarding<'a>(&'a self, domain_name: &'a str) -> url_forwarding::UrlForwardingClient<'a>`**
    *   Returns a client for managing URL Forwarding for a specific domain.
    *   **Parameters:**
        *   `domain_name: &'a str`: The domain name to operate on.
*   **`pub fn vanity_ns<'a>(&'a self, domain_name: &'a str) -> vanity_ns::VanityNameserverClient<'a>`**
    *   Returns a client for managing Vanity Nameservers for a specific domain.
    *   **Parameters:**
        *   `domain_name: &'a str`: The domain name to operate on.

#### **3.2. Sub-Clients**

##### **`name_com::domain::DomainsClient<'a>`**
Provides methods for general domain actions across an entire account. Instantiated by calling `NameDotCom::domains()`.

*   **`pub async fn list(&self) -> Result<Vec<Domain>>`**
    *   Retrieves a list of all domains in your account. This method handles pagination internally.
*   **`pub async fn check_availability(&self, domain_names: &[&str]) -> Result<Vec<AvailabilityResult>>`**
    *   Checks the availability of a list of domain names.
*   **`pub async fn create(&self, domain_name: &str) -> Result<CreateDomainResponse>`**
    *   Registers a new domain. In the dev environment, this only simulates the registration.
*   **`pub fn domain(&self, domain_name: &'a str) -> DomainClient<'a>`**
    *   Returns a client for operating on a single, specific domain.

##### **`name_com::domain::DomainClient<'a>`**
Provides methods for actions on a single specific domain. Instantiated by calling `DomainsClient::domain(...)`.

*   **`pub async fn get(&self) -> Result<Domain>`**
    *   Retrieves the details for this specific domain.
*   **`pub async fn update(&self, payload: UpdateDomainPayload) -> Result<Domain>`**
    *   Updates the autorenew, privacy, or lock status for the domain using the `PATCH` method.
*   **`pub async fn get_auth_code(&self) -> Result<String>`**
    *   Retrieves the transfer authorization code (EPP code) for a domain.
*   **`pub async fn set_nameservers(&self, nameservers: &[&str]) -> Result<Domain>`**
    *   Sets the nameservers for the domain.

##### **`name_com::dns::DnsClient<'a>`**
Provides methods for DNS/DNSSEC actions on a specific domain. Instantiated by calling `NameDotCom::dns(...)`.

*   **`pub async fn list_records(&self) -> Result<Vec<DnsRecord>>`**
    *   Retrieves a list of all DNS records for the domain. This method handles pagination internally.
*   **`pub async fn get_record(&self, record_id: i32) -> Result<DnsRecord>`**
    *   Retrieves a single DNS record by its ID.
*   **`pub async fn create_record(&self, payload: DnsRecordPayload<'_>) -> Result<DnsRecord>`**
    *   Creates a new DNS record using the provided payload.
*   **`pub async fn update_record(&self, record_id: i32, payload: DnsRecordPayload<'_>) -> Result<DnsRecord>`**
    *   Updates an existing DNS record using the `PUT` method.
*   **`pub async fn delete_record(&self, record_id: i32) -> Result<()>`**
    *   Deletes a DNS record by its ID.
*   **`pub async fn list_dnssec(&self) -> Result<Vec<DnssecRecord>>`**
    *   Retrieves a list of all DNSSEC records for the domain.
*   **`pub async fn get_dnssec(&self, digest: &str) -> Result<DnssecRecord>`**
    *   Retrieves a single DNSSEC record by its digest.
*   **`pub async fn create_dnssec(&self, payload: DnssecCreatePayload<'_>) -> Result<DnssecRecord>`**
    *   Creates a new DNSSEC record for the domain.
*   **`pub async fn delete_dnssec(&self, digest: &str) -> Result<()>`**
    *   Deletes a DNSSEC record from the domain by its digest.

##### **`name_com::url_forwarding::UrlForwardingClient<'a>`**
Provides methods for URL Forwarding actions. Instantiated by calling `NameDotCom::url_forwarding(...)`.

*   **`pub async fn list(&self) -> Result<Vec<UrlForwardingRecord>>`**
    *   Retrieves a list of all URL forwarding records for the domain. This method handles pagination internally.
*   **`pub async fn get(&self, host: &str) -> Result<UrlForwardingRecord>`**
    *   Retrieves a single URL forwarding record by its host. The `host` should be the full hostname (e.g., "www.example.org").
*   **`pub async fn create(&self, payload: UrlForwardingCreatePayload<'_>) -> Result<UrlForwardingRecord>`**
    *   Creates a new URL forwarding record.
*   **`pub async fn update(&self, host: &str, payload: UrlForwardingUpdatePayload<'_>) -> Result<UrlForwardingRecord>`**
    *   Updates an existing URL forwarding record. The `host` should be the full hostname.
*   **`pub async fn delete(&self, host: &str) -> Result<()>`**
    *   Deletes a URL forwarding record by its host. The `host` should be the full hostname.

##### **`name_com::vanity_ns::VanityNameserverClient<'a>`**
Provides methods for Vanity Nameserver actions. Instantiated by calling `NameDotCom::vanity_ns(...)`.

*   **`pub async fn list(&self) -> Result<Vec<VanityNameserver>>`**
    *   Retrieves a list of all vanity nameservers for the domain. This method handles pagination internally.
*   **`pub async fn get(&self, hostname: &str) -> Result<VanityNameserver>`**
    *   Retrieves a single vanity nameserver by its hostname.
*   **`pub async fn create(&self, payload: VanityNsCreatePayload<'_>) -> Result<VanityNameserver>`**
    *   Creates a new vanity nameserver.
*   **`pub async fn update(&self, hostname: &str, payload: VanityNsUpdatePayload<'_>) -> Result<VanityNameserver>`**
    *   Updates an existing vanity nameserver.
*   **`pub async fn delete(&self, hostname: &str) -> Result<()>`**
    *   Deletes a vanity nameserver by its hostname.

#### **3.3. Public Data Structures**

This section lists all public request and response structs for the Name.com client.

*   **`struct Hello`**: Response for a successful "hello" command.
    *   `pub motd: String`
    *   `pub server_name: String`
    *   `pub server_time: String`
    *   `pub username: String`
*   **`struct Domain`**: Represents a single domain.
    *   `pub domain_name: String`
    *   `pub create_date: String`
    *   `pub expire_date: String`
    *   `pub autorenew_enabled: bool`
    *   `pub locked: bool`
    *   `pub privacy_enabled: bool`
    *   `pub contacts: Contacts`
    *   `pub nameservers: Vec<String>`
    *   `pub renewal_price: f64`
*   **`struct Contacts`**: A placeholder for the detailed WHOIS contact information.
    *   `pub registrant: serde_json::Value`
    *   `pub admin: serde_json::Value`
    *   `pub tech: serde_json::Value`
    *   `pub billing: serde_json::Value`
*   **`struct CreateDomainResponse`**: The response after successfully creating a domain.
    *   `pub domain: Domain`
    *   `pub order: i32`
    *   `pub total_paid: f64`
*   **`struct UpdateDomainPayload`**: Payload for updating a domain's status flags. All fields are optional.
    *   `pub autorenew_enabled: Option<bool>`
    *   `pub locked: Option<bool>`
    *   `pub privacy_enabled: Option<bool>`
*   **`struct AvailabilityResult`**: The result of an availability check for a single domain.
    *   `pub domain_name: String`
    *   `pub purchasable: bool`
    *   `pub premium: bool`
    *   `pub purchase_price: f64`
    *   `pub purchase_type: String`
    *   `pub renewal_price: f64`
*   **`struct CheckAvailabilityResponse`**: The response from a domain availability check.
    *   `pub results: Vec<AvailabilityResult>`
*   **`struct DnsRecord`**: Represents a single DNS record.
    *   `pub id: i32`
    *   `pub domain_name: String`
    *   `pub host: Option<String>`
    *   `pub fqdn: String`
    *   `pub r#type: String`
    *   `pub answer: String`
    *   `pub ttl: i64`
    *   `pub priority: Option<i64>`
*   **`struct DnsRecordPayload<'a>`**: The request body for creating or updating a DNS record.
    *   `pub host: Option<&'a str>`
    *   `pub r#type: &'a str`
    *   `pub answer: &'a str`
    *   `pub ttl: i64`
    *   `pub priority: Option<i64>`
*   **`struct DnssecRecord`**: Represents a single DNSSEC record.
    *   `pub domain_name: String`
    *   `pub digest: String`
    *   `pub digest_type: i32`
    *   `pub key_tag: i32`
    *   `pub algorithm: i32`
*   **`struct DnssecCreatePayload<'a>`**: The request body for creating a new DNSSEC record.
    *   `pub digest: &'a str`
    *   `pub digest_type: i32`
    *   `pub key_tag: i32`
    *   `pub algorithm: i32`
*   **`struct UrlForwardingRecord`**: Represents a single URL forwarding record.
    *   `pub domain_name: String`
    *   `pub host: String`
    *   `pub forwards_to: String`
    *   `pub r#type: String`
    *   `pub title: Option<String>`
    *   `pub meta: Option<String>`
*   **`struct UrlForwardingCreatePayload<'a>`**: Request body for creating a URL forwarding record.
    *   `pub domain_name: &'a str`
    *   `pub host: &'a str`
    *   `pub forwards_to: &'a str`
    *   `pub r#type: &'a str`
    *   `pub title: Option<&'a str>`
    *   `pub meta: Option<&'a str>`
*   **`struct UrlForwardingUpdatePayload<'a>`**: Request body for updating a URL forwarding record.
    *   `pub forwards_to: &'a str`
    *   `pub r#type: &'a str`
    *   `pub title: Option<&'a str>`
    *   `pub meta: Option<&'a str>`
*   **`struct VanityNameserver`**: Represents a single vanity nameserver.
    *   `pub domain_name: String`
    *   `pub hostname: String`
    *   `pub ips: Vec<String>`
*   **`struct VanityNsCreatePayload<'a>`**: The request body for creating a new vanity nameserver.
    *   `pub hostname: &'a str`
    *   `pub ips: Vec<&'a str>`
*   **`struct VanityNsUpdatePayload<'a>`**: The request body for updating an existing vanity nameserver.
    *   `pub ips: Vec<&'a str>`