# Usage Guide: registrar

This guide provides a detailed overview of the `registrar` crate, covering core concepts, configuration, and practical examples for interacting with supported domain registrar APIs.

## Core Concepts

The library is designed around a central pattern: a main client for each registrar that acts as an entry point for creating more specialized, "scoped" clients.

### Main Clients
You begin by instantiating a main client for the provider you want to use, such as `porkbun::Porkbun` or `name_com::NameDotCom`. This object holds your authentication credentials and the shared HTTP client, and it manages the connection pool.

### Scoped Clients
From a main client instance, you can create temporary clients that are scoped to a specific resource, typically a domain name. This is the primary way to interact with the API. This pattern ensures that operations are always performed on the correct resource and provides a highly ergonomic API.

For example, to manage DNS records for "example.com", you would first get a scoped DNS client:
```rust
let dns_client = main_client.dns("example.com");
// Now `dns_client` can be used to list, create, or delete records for example.com
```

## Quick Start

Here are minimal, runnable examples to get you started quickly.

### Example: Porkbun
This example authenticates with the Porkbun API, tests the connection using `ping`, and then retrieves all DNS records for a specific domain.

```rust,no_run
use registrar::porkbun::Porkbun;
use registrar::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load credentials from environment variables or a config file
    let apikey = std::env::var("PORKBUN_API_KEY").expect("PORKBUN_API_KEY not set");
    let secret = std::env::var("PORKBUN_SECRET_KEY").expect("PORKBUN_SECRET_KEY not set");
    let domain = "example.com";

    // 1. Create the main Porkbun client
    let client = Porkbun::new(apikey, secret);

    // 2. Test authentication and connectivity
    let ping_response = client.ping().await?;
    println!("Successfully connected to Porkbun. Your IP: {}", ping_response.your_ip);

    // 3. Get a scoped client for the domain's DNS records
    let dns_client = client.dns(domain);

    // 4. Perform an action with the scoped client
    let records = dns_client.retrieve_all_records().await?;
    println!("Found {} DNS records for {}", records.len(), domain);
    for record in records {
        println!("  - [{}] {} -> {}", record.r#type, record.name, record.content);
    }

    Ok(())
}
```

### Example: Name.com
This example connects to the Name.com development (sandbox) environment, tests the connection, and then lists all domains in the account.

```rust,no_run
use registrar::name_com::NameDotCom;
use registrar::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load credentials for the development environment
    let username = std::env::var("NAMECOM_DEV_USERNAME").expect("NAMECOM_DEV_USERNAME not set");
    let token = std::env::var("NAMECOM_DEV_TOKEN").expect("NAMECOM_DEV_TOKEN not set");

    // 1. Create the main Name.com client for the dev environment
    let client = NameDotCom::new_dev(username, token);

    // 2. Test authentication and connectivity
    let hello_response = client.hello().await?;
    println!("Successfully connected to Name.com. Logged in as: {}", hello_response.username);

    // 3. Get a scoped client for general domain actions
    let domains_client = client.domains();

    // 4. Perform an action
    let domains = domains_client.list().await?;
    println!("Found {} domains in the account.", domains.len());
    for domain in domains {
        println!("  - {} (Expires: {})", domain.domain_name, domain.expire_date);
    }

    Ok(())
}
```

## Main API Sections

### Porkbun (`registrar::porkbun`)

#### `Porkbun` (Main Client)
The entry point for the Porkbun API.

**Methods:**
- `Porkbun::new(apikey: String, secretapikey: String) -> Self`
  - Creates a new client for the production API.
- `ping(&self) -> Result<PingResponse>`
  - Tests authentication and connectivity.
- `get_pricing(&self) -> Result<PricingResponse>`
  - Retrieves pricing for all TLDs.
- `domain<'a>(&'a self, domain: &'a str) -> domain::Domain<'a>`
  - Gets a scoped client for domain-level actions.
- `dns<'a>(&'a self, domain: &'a str) -> dns::Dns<'a>`
  - Gets a scoped client for DNS record management.
- `ssl<'a>(&'a self, domain: &'a str) -> ssl::Ssl<'a>`
  - Gets a scoped client for SSL bundle retrieval.

#### `domain::Domain` (Scoped Client)
Manages nameservers, URL forwarding, and glue records for a specific domain.

**Common Methods:**
- `update_nameservers(&self, nameservers: &[&str]) -> Result<StatusResponse>`
  - Updates the domain's nameservers.
- `get_url_forwarding(&self) -> Result<Vec<UrlForwardRecord>>`
  - Retrieves all URL forwarding records.
- `create_glue_record(&self, subdomain: &str, ips: &[IpAddr]) -> Result<StatusResponse>`
  - Creates a new glue record (vanity nameserver).
- `list_all(&self, include_labels: bool) -> Result<Vec<DomainInfo>>`
  - Retrieves all domains in the account.

#### `dns::Dns` (Scoped Client)
Manages DNS and DNSSEC records for a specific domain.

**Common Methods:**
- `create_record(&self, options: DnsRecordCreateOptions<'_>) -> Result<DnsRecordCreateResponse>`
  - Creates a new DNS record.
- `retrieve_all_records(&self) -> Result<Vec<DnsRecord>>`
  - Retrieves all DNS records for the domain.
- `delete_record_by_id(&self, record_id: u64) -> Result<StatusResponse>`
  - Deletes a DNS record by its ID.
- `get_dnssec_records(&self) -> Result<HashMap<String, DnssecRecord>>`
  - Retrieves all DNSSEC records.

---

### Name.com (`registrar::name_com`)

#### `NameDotCom` (Main Client)
The entry point for the Name.com Core API.

**Methods:**
- `NameDotCom::new(username: String, token: String) -> Self`
  - Creates a new client for the **production** API.
- `NameDotCom::new_dev(username: String, token: String) -> Self`
  - Creates a new client for the **development** (sandbox) API.
- `hello(&self) -> Result<Hello>`
  - Tests authentication and connectivity.
- `domains<'a>(&'a self) -> domain::DomainsClient<'a>`
  - Gets a client for account-level domain actions.
- `dns<'a>(&'a self, domain_name: &'a str) -> dns::DnsClient<'a>`
  - Gets a scoped client for DNS/DNSSEC management.
- `url_forwarding<'a>(&'a self, domain_name: &'a str) -> url_forwarding::UrlForwardingClient<'a>`
  - Gets a scoped client for URL forwarding.
- `vanity_ns<'a>(&'a self, domain_name: &'a str) -> vanity_ns::VanityNameserverClient<'a>`
  - Gets a scoped client for vanity nameservers.

#### `domain::DomainsClient` & `domain::DomainClient`
Manages domain registration, listing, and configuration.

**Common Methods:**
- `domains().list(&self) -> Result<Vec<Domain>>`
  - Retrieves all domains in the account.
- `domains().create(&self, domain_name: &str) -> Result<CreateDomainResponse>`
  - Registers a new domain.
- `domains().domain("example.com").get(&self) -> Result<Domain>`
  - Retrieves detailed information for a specific domain.
- `domains().domain("example.com").update(&self, payload: UpdateDomainPayload) -> Result<Domain>`
  - Updates a domain's lock, privacy, or autorenew status.
- `domains().domain("example.com").set_nameservers(&self, nameservers: &[&str]) -> Result<Domain>`
  - Sets the nameservers for a specific domain.

#### `dns::DnsClient` (Scoped Client)
Manages DNS and DNSSEC records for a specific domain.

**Common Methods:**
- `create_record(&self, payload: DnsRecordPayload<'_>) -> Result<DnsRecord>`
  - Creates a new DNS record.
- `list_records(&self) -> Result<Vec<DnsRecord>>`
  - Retrieves all DNS records for the domain.
- `delete_record(&self, record_id: i32) -> Result<()>`
  - Deletes a DNS record by its ID.
- `list_dnssec(&self) -> Result<Vec<DnssecRecord>>`
  - Retrieves all DNSSEC records.

## Error Handling

All fallible operations in the crate return a `registrar::Result<T>`, which is an alias for `std::result::Result<T, registrar::Error>`.

The primary error type is the `registrar::Error` enum, which has three variants:

- **`Error::Http(reqwest::Error)`**: A lower-level error occurred during the network request itself (e.g., connection refused, DNS lookup failure, invalid TLS certificate).
- **`Error::Json(serde_json::Error)`**: An error occurred while serializing the request data to JSON or deserializing the response body from JSON. This often indicates a malformed response from the API or a bug in the library's data structures.
- **`Error::Api(String)`**: The API server successfully received and processed the request but returned a logical error (e.g., "Invalid API Key", "Domain not available", "Unknown API endpoint"). The `String` contains the descriptive error message from the provider.