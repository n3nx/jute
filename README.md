<!-- Project Metadata -->
<!-- project_tags: jute, jwt, jose, n3n -->
<!-- project_featured: true -->

# 🌿 jute

[![N3NX](https://img.shields.io/badge/n3n-org-%23666bff.svg)](https://n3n.org)
[![N3NX](https://img.shields.io/badge/discord-n3n-%237289da.svg?logo=discord)](https://discord.gg/kTWsyk5eV6)
[![Crates.io](https://img.shields.io/crates/v/jute.svg)](https://crates.io/crates/jute)
[![Docs](https://docs.rs/jute/badge.svg)](https://docs.rs/jute)
[![dependency status](https://deps.rs/repo/github/n3nx/jute/status.svg)](https://deps.rs/repo/github/n3nx/jute)
[![Build status](https://github.com/n3nx/jute/workflows/CI/badge.svg)](https://github.com/n3nx/jute/actions)

An opinionated JSON Object Signing Encryption (JOSE) library that includes implementations of JSON Web Tokens (JWT), JSON Web Signature (JWS), JSON Web Encryption (JWE) along with their dependencies like JSON Web Algorithm (JWA) and JSON Web Keys (JWK).

This work is originally derived from [`biscuit`](https://github.com/lawliet89/biscuit).

## Motivation

The name *Jute* is a phonetic term of how *JWT* sounds.

Our primary focus for this project is to ensure that the algorithms and techniques used in this library are cryptographically secure and reliable for present and future use.

Since this library is opinionated, we have removed implementations that are mentioned in RFCs but are considered obsolete and/or unsecure to use by today's standards.
See the [documentation](https://github.com/n3nx/jute/blob/dev/doc/supported.md) for more information.

Contributions and peer reviews are highly appreciated and we welcome everyone who wants to support this project.

## Contribution

[![N3N Contributor Guidelines](https://img.shields.io/badge/N3N%20Guidelines-v1.0-ff69b4.svg)](./CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in a N3N project or a Knytx Labs open source project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

## Installation

Add the following to Cargo.toml:

```toml
jute = "0.2.0"
```

To use the latest `master` branch, for example:

```toml
jute = { git = "https://github.com/n3nx/jute", branch = "dev" }
```

### License

This contribution is dual licensed under EITHER OF

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to the N3N contributors, NCRYPTO Labs Private Limited or any other licensee/user of the contribution.
