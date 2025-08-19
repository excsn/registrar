//! Contains all v3 API endpoint constants for Porkbun.

// The base URL for all API v3 calls.
pub const BASE_URL: &str = "https://api.porkbun.com/api/json/v3";

// --- General Endpoints ---
pub const PING: &str = "/ping";
pub const PRICING_GET: &str = "/pricing/get";

// --- Domain Endpoints ---
// Note: URLs with path parameters will be concatenated with the relevant domain/ID.
pub const DOMAIN_UPDATE_NS: &str = "/domain/updateNs/";
pub const DOMAIN_GET_NS: &str = "/domain/getNs/";
pub const DOMAIN_LIST_ALL: &str = "/domain/listAll";
pub const DOMAIN_ADD_URL_FORWARD: &str = "/domain/addUrlForward/";
pub const DOMAIN_GET_URL_FORWARDING: &str = "/domain/getUrlForwarding/";
pub const DOMAIN_DELETE_URL_FORWARD: &str = "/domain/deleteUrlForward/";
pub const DOMAIN_CHECK: &str = "/domain/checkDomain/";
pub const DOMAIN_CREATE_GLUE: &str = "/domain/createGlue/";
pub const DOMAIN_UPDATE_GLUE: &str = "/domain/updateGlue/";
pub const DOMAIN_DELETE_GLUE: &str = "/domain/deleteGlue/";
pub const DOMAIN_GET_GLUE: &str = "/domain/getGlue/";

// --- DNS Endpoints ---
pub const DNS_CREATE: &str = "/dns/create/";
pub const DNS_EDIT_BY_ID: &str = "/dns/edit/";
pub const DNS_EDIT_BY_NAME_TYPE: &str = "/dns/editByNameType/";
pub const DNS_DELETE_BY_ID: &str = "/dns/delete/";
pub const DNS_DELETE_BY_NAME_TYPE: &str = "/dns/deleteByNameType/";
pub const DNS_RETRIEVE_BY_DOMAIN: &str = "/dns/retrieve/"; // Also used for retrieve by ID
pub const DNS_RETRIEVE_BY_NAME_TYPE: &str = "/dns/retrieveByNameType/";
pub const DNSSEC_CREATE: &str = "/dns/createDnssecRecord/";
pub const DNSSEC_GET: &str = "/dns/getDnssecRecords/";
pub const DNSSEC_DELETE: &str = "/dns/deleteDnssecRecord/";

// --- SSL Endpoints ---
pub const SSL_RETRIEVE_BUNDLE: &str = "/ssl/retrieve/";