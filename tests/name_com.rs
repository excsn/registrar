mod common;

use registrar::name_com::{
  dns::types::DnsRecordPayload, url_forwarding::types::UrlForwardingCreatePayload,
  vanity_ns::types::VanityNsCreatePayload,
};
use uuid::Uuid;

#[tokio::test]
async fn hello_succeeds() {
  let client = common::namecom_dev_client();
  let result = client.hello().await;
  assert!(
    result.is_ok(),
    "Name.com dev environment 'hello' failed: {:?}",
    result.err()
  );
  let response = result.unwrap();
  println!(
    "Name.com 'hello' successful. Logged in as: {}. Server: {}",
    response.username, response.server_name
  );
}

// Helper to ensure the test domain exists in the sandbox.
async fn ensure_domain_exists(client: &registrar::name_com::NameDotCom, domain_name: &str) {
  println!(
    "[Prerequisite] Ensuring domain '{}' exists in dev environment...",
    domain_name
  );
  let create_domain_result = client.domains().create(domain_name).await;
  if let Err(e) = &create_domain_result {
    let error_message = e.to_string().to_lowercase();
    if !error_message.contains("domain already exists") && !error_message.contains("domain is not available") {
      panic!("Failed to create domain for an unexpected reason: {:?}", e);
    }
  }
}

#[tokio::test]
async fn can_create_and_get_dns_record() {
  // --- SETUP ---
  let client = common::namecom_dev_client();
  let config = common::get_config().name_com;
  let domain_name = &config.domain;
  ensure_domain_exists(&client, domain_name).await;

  let dns_client = client.dns(domain_name);
  let unique_value = Uuid::new_v4().to_string();
  let record_host = "integration-test-dns";

  let payload = DnsRecordPayload {
    host: Some(record_host),
    r#type: "TXT",
    answer: &unique_value,
    ttl: 300,
    priority: None,
  };

  let new_record = dns_client
    .create_record(payload)
    .await
    .expect("SETUP FAILED: Could not create DNS record.");
  let new_record_id = new_record.id;
  println!("[Setup] Created Name.com DNS record with ID: {}", new_record_id);

  // The guard's lifetime is tied to the test scope.
  let _guard = common::NameComDnsRecordGuard::new(new_record_id, domain_name, &client);

  // --- EXECUTE & ASSERT ---
  let record = dns_client
    .get_record(new_record_id)
    .await
    .expect("Failed to retrieve DNS record by ID.");

  assert_eq!(record.id, new_record_id);
  assert_eq!(record.answer, unique_value);
  println!("[Test] Verification successful for DNS record.");
}

#[tokio::test]
async fn can_create_and_get_url_forward() {
  // --- SETUP ---
  let client = common::namecom_dev_client();
  let config = common::get_config().name_com;
  let domain_name = &config.domain;
  ensure_domain_exists(&client, domain_name).await;

  let fwd_client = client.url_forwarding(domain_name);
  let subdomain = format!("test-fwd-{}", Uuid::new_v4().to_string()[..8].to_string());
  let payload = UrlForwardingCreatePayload {
    domain_name,
    host: &subdomain,
    forwards_to: "https://name.com",
    r#type: "redirect",
    title: None,
    meta: None,
  };

  let new_forward = fwd_client
    .create(payload)
    .await
    .expect("SETUP FAILED: Could not create URL forward.");
  println!("[Setup] Created URL Forward for host: {}", new_forward.host);

  // The guard's lifetime is tied to the test scope.
  let _guard = common::NameComUrlForwardGuard::new(subdomain.clone(), domain_name, &client);

  // --- EXECUTE & ASSERT ---
  let retrieved_forward = fwd_client
    .get(&subdomain)
    .await
    .expect("Failed to retrieve URL Forward by host.");
  assert_eq!(retrieved_forward.host, subdomain);
  assert_eq!(retrieved_forward.forwards_to, "https://name.com");
  println!("[Test] Verification successful for URL forward.");
}

#[tokio::test]
async fn can_create_and_get_vanity_nameserver() {
  // --- SETUP ---
  let client = common::namecom_dev_client();
  let config = common::get_config().name_com;
  let domain_name = &config.domain;
  ensure_domain_exists(&client, domain_name).await;

  let vns_client = client.vanity_ns(domain_name);
  let subdomain = format!("ns1-test-{}", Uuid::new_v4().to_string()[..8].to_string());
  let full_hostname = format!("{}.{}", subdomain, domain_name);
  let ips = vec!["1.2.3.4", "5.6.7.8"];

  let payload = VanityNsCreatePayload {
    hostname: &subdomain,
    ips: ips.clone(),
  };

  let new_vns = vns_client
    .create(payload)
    .await
    .expect("SETUP FAILED: Could not create Vanity NS.");
  println!("[Setup] Created Vanity NS for hostname: {}", new_vns.hostname);

  // The guard's lifetime is tied to the test scope.
  let _guard = common::NameComVanityNsGuard::new(full_hostname.clone(), domain_name, &client);

  // --- EXECUTE & ASSERT ---
  let retrieved_vns = vns_client
    .get(&full_hostname)
    .await
    .expect("Failed to retrieve Vanity NS by hostname.");
  assert_eq!(retrieved_vns.hostname, full_hostname);
  assert_eq!(retrieved_vns.ips, ips);
  println!("[Test] Verification successful for Vanity NS.");
}
