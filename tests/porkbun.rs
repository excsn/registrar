mod common;

use std::str::FromStr;
use std::{net::IpAddr, time::Duration};

use registrar::porkbun::{dns::types::DnsRecordCreateOptions, domain::types::UrlForwardRecord};
use uuid::Uuid;

#[tokio::test]
async fn ping_succeeds() {
  let client = common::porkbun_client();
  let result = client.ping().await;
  assert!(result.is_ok(), "Porkbun ping failed: {:?}", result.err());
  println!("Porkbun ping successful. Your IP is: {}", result.unwrap().your_ip);
}

#[tokio::test]
async fn can_create_and_get_dns_record() {
  // --- SETUP ---
  let client = common::porkbun_client();
  let config = common::get_config().porkbun;
  let dns_client = client.dns(&config.domain);
  let unique_value = Uuid::new_v4().to_string();
  let record_name = Some("integration-test-dns");

  let options = DnsRecordCreateOptions {
    name: record_name,
    r#type: "TXT",
    content: &unique_value,
    ttl: Some("300"),
    prio: None,
  };

  let create_result = dns_client
    .create_record(options)
    .await
    .expect("SETUP FAILED: Could not create DNS record.");
  let new_record_id = create_result.id;
  println!("[Setup] Created Porkbun DNS record with ID: {}", new_record_id);

  // The guard's lifetime is tied to the test scope.
  let _guard = common::PorkbunDnsRecordGuard::new(new_record_id, &config.domain, &client);

  // --- EXECUTE & ASSERT ---
  let retrieved_record = dns_client
    .retrieve_record_by_id(new_record_id)
    .await
    .expect("Failed to retrieve DNS record by ID.")
    .expect("Expected to find record by ID, but it was not found.");

  assert_eq!(retrieved_record.id, new_record_id.to_string());
  assert_eq!(retrieved_record.content, unique_value);
  println!("[Test] Verification successful for DNS record.");
}

#[tokio::test]
async fn can_create_and_get_url_forward() {
  // --- SETUP ---
  let client = common::porkbun_client();
  let config = common::get_config().porkbun;
  let domain_client = client.domain(&config.domain);
  let unique_subdomain = format!("test-fwd-{}", Uuid::new_v4().to_string()[..8].to_string());

  let options = UrlForwardRecord {
    id: "0".to_string(),
    subdomain: unique_subdomain.clone(),
    location: "https://google.com".to_string(),
    r#type: "temporary".to_string(),
    include_path: "yes".to_string(),
    wildcard: "no".to_string(),
  };

  domain_client
    .add_url_forward(&options)
    .await
    .expect("SETUP FAILED: Could not create URL forward.");
  println!("[Setup] Created URL forward for subdomain: {}", unique_subdomain);

  // --- EXECUTE & ASSERT ---
  let forwards = domain_client
    .get_url_forwarding()
    .await
    .expect("Failed to retrieve URL forwards.");
  let record = forwards
    .into_iter()
    .find(|rec| rec.subdomain == unique_subdomain)
    .expect("Could not find the newly created URL forward record.");

  assert_eq!(record.location, "https://google.com");
  println!("[Test] Verification successful for URL forward with ID: {}", record.id);

  // The guard's lifetime is tied to the test scope.
  let record_id = record.id.parse::<u64>().expect("Failed to parse record ID");
  let _guard = common::PorkbunUrlForwardGuard::new(record_id, &config.domain, &client);
}

#[tokio::test]
async fn can_create_and_get_glue_record() {
  // --- SETUP ---
  let client = common::porkbun_client();
  let config = common::get_config().porkbun;
  let domain_client = client.domain(&config.domain);
  let subdomain = format!("ns-drop-{}", Uuid::new_v4().to_string()[..8].to_string());
  let full_hostname = format!("{}.{}", subdomain, config.domain);
  let ips = vec![
    IpAddr::from_str("1.1.1.1").unwrap(),
    IpAddr::from_str("2001:4860:4860::8888").unwrap(),
  ];

  domain_client
    .create_glue_record(&subdomain, &ips)
    .await
    .expect("SETUP FAILED: Could not create glue record.");
  println!("[Setup] Creating Porkbun glue record: {}", subdomain);

  // The guard is created AFTER the resource exists.
  let _guard = common::PorkbunGlueRecordGuard::new(subdomain, &config.domain, &client);

  // --- EXECUTE & ASSERT ---
  println!("[Test] Verifying glue record {} exists...", _guard.subdomain);
  tokio::time::sleep(Duration::from_secs(3)).await;

  let all_records = domain_client
    .get_glue_records()
    .await
    .expect("Failed to get glue records");
  let maybe_record = all_records
    .into_iter()
    .find(|(hostname, _)| hostname.eq_ignore_ascii_case(&full_hostname));

  assert!(maybe_record.is_some(), "Could not find the newly created glue record.");
  println!("[Test] Verification successful for glue record.");
}
