# registrar

[![Crates.io](https://img.shields.io/crates/v/registrar.svg)](https://crates.io/crates/registrar)
[![Docs.rs](https://docs.rs/registrar/badge.svg)](https://docs.rs/registrar)
[![License](https://img.shields.io/crates/l/registrar.svg)](https://github.com/excsn/registrar/blob/main/LICENSE)

A unified, asynchronous, and strongly-typed client for domain registrar APIs. This crate provides a clean and consistent Rust interface for interacting with various registrars, abstracting away the differences in their specific API protocols. It allows developers to manage domains, DNS records, and other registrar services without needing to write clients for each provider.

## Key Features

### Multi-Provider Support
Interact with multiple domain registrars through a single, consistent library. Each registrar is enabled via a Cargo feature flag, ensuring you only compile the code you need.
- `porkbun`: Full support for the Porkbun v3 API.
- `name-com`: Full support for the Name.com Core API.

### Strongly-Typed & Asynchronous
All API requests and responses are mapped to robust Rust structs, providing compile-time safety and leveraging `serde` for reliable serialization and deserialization. The entire library is built on `async/await`, making it non-blocking and suitable for high-performance applications.

### Scoped Client Design
The library uses an ergonomic "scoped client" pattern. After instantiating a main provider client (e.g., `Porkbun::new(...)`), you can create temporary, specialized clients for specific domains (e.g., `client.dns("example.com")`). This design provides a safe and intuitive way to manage resources.

## Installation

Add `registrar` to your `Cargo.toml` file. By default, both `porkbun` and `name-com` clients are enabled.

```toml
[dependencies]
registrar = "0.9.0"
```

To enable only specific providers and reduce dependencies, disable the default features and specify the ones you need:

```toml
[dependencies]
registrar = { version = "0.9.0", default-features = false, features = ["porkbun"] }
```

## Documentation

For a detailed guide on core concepts, configuration, and complete, runnable examples, please see the **[Usage Guide (README.USAGE.md)](README.USAGE.md)**.

The full, comprehensive API reference is available on [docs.rs](https://docs.rs/registrar).

## License

This library is distributed under the terms of the **MPL-2.0** License.