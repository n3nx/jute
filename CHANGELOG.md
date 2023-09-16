# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2023-09-16

[0.1.2]: ../../../releases/tag/v0.1.2

### Bug Fixes

- *Cargo.toml:* Limit serde version to upper limit of 1.0.172. ([ede8707](ede87076555b15c51e1196f9e443dcd49abe12e5))

### Documentation

- *Cargo.toml:* Shorten project description ([7673c09](7673c0937d3ebac73d260612c6bd77695a034a6b))
- *jute:* Fix jute crates documentation and resource links ([eb46ca4](eb46ca4b51fc17abc7536cc92aaed8d6a09850df))

### Features

- *jute:* Create project base ([e9f89db](e9f89dbca8057a38ca3416f1ede990965e8ce97a))
- *jute:* Derive substrate from project and fork https://github.com/lawliet89/biscuit ([b5d837a](b5d837a6e6961d265aeb8b9a1b0730722e72561c))

### Improvements

- *jute:* Replace `data_encoding` with `base64ct` for constant time operations ([75b5635](75b5635ca984e92f9b493f612514014b5b66f259))
- *jute:* Substitute crate `chrono` with `time` because of stable apis and good maintainance record ([cacd67c](cacd67c382e5529ae797290d44f781948fea101c))

### Refactoring Updates

- *jute:* Bump version to patch v0.1.1 ([d966d0d](d966d0d053ce82a71726bef916c90d39bf652441))

<!-- CHANGELOG SPLIT MARKER -->
