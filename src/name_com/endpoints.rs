//! Contains all API endpoint constants for Name.com.
//! Note: Paths that require parameters like a domain name are treated as prefixes.


// --- Core API v1 Endpoints ---

pub const HELLO: &str = "/core/v1/hello";

pub const CORE_V1_DOMAINS_PREFIX: &str = "/core/v1/domains/"; // Requires {domainName}
pub const CORE_V1_RECORDS_SUFFIX: &str = "/records"; // Appended to CORE_V1_DOMAINS_PREFIX path
pub const CORE_V1_DNSSEC_SUFFIX: &str = "/dnssec"; // Appended to CORE_V1_DOMAINS_PREFIX path
pub const CORE_V1_URL_FORWARDING_SUFFIX: &str = "/url/forwarding"; // Appended to CORE_V1_DOMAINS_PREFIX path
pub const CORE_V1_VANITY_NS_SUFFIX: &str = "/vanity_nameservers"; // Appended to CORE_V1_DOMAINS_PREFIX path

// --- Core API v1 Actions (appended to domain paths) ---
pub const CORE_V1_ACTION_GET_AUTH_CODE: &str = ":getAuthCode";
pub const CORE_V1_ACTION_SET_NAMESERVERS: &str = ":setNameservers";
pub const CORE_V1_ACTION_CHECK_AVAILABILITY: &str = ":checkAvailability";
