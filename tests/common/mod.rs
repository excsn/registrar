// tests/common/mod.rs

use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

use c5store::{C5Store, C5StoreOptions, create_c5store};
use ctor::{ctor, dtor};
use once_cell::sync::{Lazy, OnceCell};
use registrar::{name_com::NameDotCom, porkbun::Porkbun};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::runtime::Handle;
use tokio::sync::mpsc;

// --- Configuration Structs ---
// These structs map directly to the YAML configuration files.

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TestConfig {
  pub porkbun: PorkbunConfig,
  #[serde(rename = "name_com")]
  pub name_com: NameComConfig,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PorkbunConfig {
  pub domain: String,
  pub credentials: PorkbunCredentials,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PorkbunCredentials {
  pub apikey: String,
  pub secretapikey: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NameComConfig {
  pub domain: String,
  pub credentials: NameComCredentials,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NameComCredentials {
  pub username: String,
  pub token: String,
}

// --- C5Store Initialization ---

static STORE: Lazy<c5store::C5StoreRoot> = Lazy::new(|| {
  let config_paths = vec![
    PathBuf::from("tests/config/default.yaml"),
    // This file is git-ignored and provides local overrides.
    // c5store will not error if it's missing, making it optional.
    PathBuf::from("tests/config/local.yaml"),
  ];

  // Using default options is fine for our test setup.
  // This will automatically handle environment variable overrides (e.g., C5_PORKBUN_DOMAIN=...).
  let options = C5StoreOptions::default();

  let (store, _mgr) = create_c5store(config_paths, Some(options)).expect("Failed to load test configuration");

  store
});

// --- Public Helper Functions ---

/// Retrieves and deserializes the entire test configuration.
/// Panics if the configuration is not structured correctly.
pub fn get_config() -> TestConfig {
  STORE
    .get_into_struct("")
    .expect("Failed to deserialize configuration into TestConfig struct")
}

/// Creates a Porkbun client pre-configured from the loaded test settings.
pub fn porkbun_client() -> Porkbun {
  let config = get_config();
  Porkbun::new(
    config.porkbun.credentials.apikey,
    config.porkbun.credentials.secretapikey,
  )
}

/// Creates a Name.com client pre-configured for the DEV environment
/// from the loaded test settings.
pub fn namecom_dev_client() -> NameDotCom {
  let config = get_config();
  NameDotCom::new_dev(config.name_com.credentials.username, config.name_com.credentials.token)
}

// --- Test Utilities ---
// These guards now use an explicit, awaitable `clean` method for teardown.

// --- PORKBUN GUARDS ---

pub struct PorkbunDnsRecordGuard {
  pub id: u64,
  pub domain_name: String,
  client: Porkbun,
}
impl PorkbunDnsRecordGuard {
  pub fn new(id: u64, domain_name: &str, client: &Porkbun) -> Self {
    Self {
      id,
      domain_name: domain_name.to_string(),
      client: client.clone(),
    }
  }
  pub async fn clean(self) {
    println!("Cleaning up Porkbun DNS record ID: {}", self.id);
    let dns_client = self.client.dns(&self.domain_name);
    if let Err(e) = dns_client.delete_record_by_id(self.id).await {
      eprintln!("Failed to clean up Porkbun DNS record {}: {:?}", self.id, e);
    }
  }
}

pub struct PorkbunUrlForwardGuard {
  pub id: u64,
  pub domain_name: String,
  client: Porkbun,
}
impl PorkbunUrlForwardGuard {
  pub fn new(id: u64, domain_name: &str, client: &Porkbun) -> Self {
    Self {
      id,
      domain_name: domain_name.to_string(),
      client: client.clone(),
    }
  }
  pub async fn clean(self) {
    println!("Cleaning up Porkbun URL Forward record ID: {}", self.id);
    let domain_client = self.client.domain(&self.domain_name);
    if let Err(e) = domain_client.delete_url_forward(self.id).await {
      eprintln!("Failed to clean up Porkbun URL Forward record {}: {:?}", self.id, e);
    }
  }
}

pub struct PorkbunGlueRecordGuard {
  pub subdomain: String,
  pub domain_name: String,
  client: Porkbun,
}
impl PorkbunGlueRecordGuard {
  pub fn new(subdomain: String, domain_name: &str, client: &Porkbun) -> Self {
    Self {
      subdomain,
      domain_name: domain_name.to_string(),
      client: client.clone(),
    }
  }
}

impl Drop for PorkbunGlueRecordGuard {
  fn drop(&mut self) {
    let subdomain = self.subdomain.clone();
    let domain_name = self.domain_name.clone();
    let client = self.client.clone();
    let cleanup_future = async move {
      println!("[Teardown] Cleaning up Porkbun Glue Record: {}", subdomain);
      let domain_client = client.domain(&domain_name);
      if let Err(e) = domain_client.delete_glue_record(&subdomain).await {
        eprintln!(
          "[Teardown] FAILED to clean up Porkbun Glue Record {}: {:?}",
          subdomain, e
        );
      }
      println!("[Teardown] Cleaned up Porkbun Glue Record: {}", subdomain);
    };
    // Use the new, simpler registration function.
    register_teardown(Box::pin(cleanup_future));
  }
}

// --- NAME.COM GUARDS ---

pub struct NameComDnsRecordGuard {
  pub id: i32,
  pub domain_name: String,
  client: NameDotCom,
}
impl NameComDnsRecordGuard {
  pub fn new(id: i32, domain_name: &str, client: &NameDotCom) -> Self {
    Self {
      id,
      domain_name: domain_name.to_string(),
      client: client.clone(),
    }
  }
  pub async fn clean(self) {
    println!("Cleaning up Name.com DNS record ID: {}", self.id);
    let dns_client = self.client.dns(&self.domain_name);
    if let Err(e) = dns_client.delete_record(self.id).await {
      eprintln!("Failed to clean up Name.com DNS record {}: {:?}", self.id, e);
    }
  }
}

pub struct NameComUrlForwardGuard {
  pub host: String, // This is the subdomain
  pub domain_name: String,
  client: NameDotCom,
}
impl NameComUrlForwardGuard {
  pub fn new(host: String, domain_name: &str, client: &NameDotCom) -> Self {
    Self {
      host,
      domain_name: domain_name.to_string(),
      client: client.clone(),
    }
  }
  pub async fn clean(self) {
    println!("Cleaning up Name.com URL Forward for host: {}", self.host);
    let fwd_client = self.client.url_forwarding(&self.domain_name);
    if let Err(e) = fwd_client.delete(&self.host).await {
      eprintln!("Failed to clean up Name.com URL Forward {}: {:?}", self.host, e);
    }
  }
}

pub struct NameComVanityNsGuard {
  pub hostname: String, // This is the full hostname
  pub domain_name: String,
  client: NameDotCom,
}
impl NameComVanityNsGuard {
  pub fn new(hostname: String, domain_name: &str, client: &NameDotCom) -> Self {
    Self {
      hostname,
      domain_name: domain_name.to_string(),
      client: client.clone(),
    }
  }
  pub async fn clean(self) {
    println!("Cleaning up Name.com Vanity NS: {}", self.hostname);
    let vns_client = self.client.vanity_ns(&self.domain_name);
    if let Err(e) = vns_client.delete(&self.hostname).await {
      eprintln!("Failed to clean up Name.com Vanity NS {}: {:?}", self.hostname, e);
    }
  }
}

// --- Teardown Registry ---

type Job = Pin<Box<dyn Future<Output = ()> + Send>>;

/// The state that will be protected by the Mutex.
struct TeardownState {
  sender: Option<mpsc::Sender<Job>>,
  worker_handle: Option<JoinHandle<()>>,
}

// The global singleton is now a Mutex wrapping our state.
static REGISTRY: Lazy<Mutex<TeardownState>> = Lazy::new(|| {
  let (sender, mut receiver) = mpsc::channel::<Job>(128);

  let worker_handle = thread::spawn(move || {
    let rt = tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .build()
      .expect("Failed to create cleanup worker runtime");

    rt.block_on(async move {
      println!("[Registry] Cleanup worker thread started.");

      // A vector to hold the handles of all spawned cleanup tasks.
      let mut tasks = Vec::new();

      // This loop receives jobs and spawns them, collecting the handles.
      while let Some(job) = receiver.recv().await {
        let task_handle = tokio::spawn(job);
        tasks.push(task_handle);
      }

      println!(
        "[Registry] All tests finished. Waiting for {} cleanup tasks to complete...",
        tasks.len()
      );

      // THIS IS THE CRITICAL FIX:
      // After the channel is closed, we wait for all spawned tasks to finish.
      for task in tasks {
        if let Err(e) = task.await {
          eprintln!("[Registry] A cleanup task panicked: {:?}", e);
        }
      }

      println!("[Registry] All cleanup tasks completed, worker shutting down.");
    });
  });

  // This part remains the same.
  Mutex::new(TeardownState {
    sender: Some(sender),
    worker_handle: Some(worker_handle),
  })
});

/// This is a one-time setup function that just ensures the lazy static is initialized.
#[ctor]
fn global_setup() {
  println!("[Registry] Global setup running (ctor)...");
  // Accessing the registry for the first time will initialize it.
  REGISTRY.lock().unwrap();
}

/// This is our global teardown function. It runs ONCE before the test process exits.
#[dtor]
fn global_teardown() {
  println!("[Registry] Global teardown running (dtor)...");
  // Lock the mutex to get mutable access to the state.
  let mut state = REGISTRY.lock().unwrap();

  // 1. Take the sender out of the Option and drop it. This closes the channel.
  if let Some(sender) = state.sender.take() {
    drop(sender);
  }

  // 2. Take the worker handle.
  if let Some(handle) = state.worker_handle.take() {
    // 3. Now that the channel is closed, the worker is guaranteed to terminate.
    //    `join()` will not deadlock.
    handle.join().expect("Cleanup worker thread panicked");
    println!("[Registry] Teardown complete. Process will now exit.");
  }
}

// The registration function locks the mutex to get access to the sender.
fn register_teardown(job: Job) {
  let state = REGISTRY.lock().unwrap();
  if let Some(sender) = &state.sender {
    if let Err(e) = sender.try_send(job) {
      eprintln!(
        "[Registry] FAILED to register teardown job: {}. Cleanup may not run.",
        e
      );
    }
  }
}
