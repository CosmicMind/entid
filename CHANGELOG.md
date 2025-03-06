# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2024-05-15

### Added
- Added more flexible string handling with `AsRef<str>` for string parameters
- Added standard Rust traits: `TryFrom`, `FromStr`, `Borrow<str>`, and `AsRef<str>`
- Added convenience methods: `with_uuid`, `new_v4`, `new_v5`, `with_ulid`, `with_timestamp`, and `monotonic_from`
- Added builder pattern with `EntityIdBuilder` for more flexible creation of entity IDs
- Added thread-safe caching for string representations in `Identifier` types
- Added `id_str()` method to get the raw identifier string without prefix

### Changed
- Improved API consistency with `with_` prefix for builder methods
- Updated documentation with examples of the new flexible API

## [0.3.0] - 2024-03-05

### Changed
- Updated the derive macro attribute format to use a single `#[entid(...)]` attribute
- Removed separate `#[prefix]` and `#[delimiter]` attributes
- Updated examples and documentation to reflect the new attribute format
- Fixed compatibility with the latest version of the `syn` crate

## [0.2.0] - 2024-03-05

### Added
- Added derive macro for implementing the `Prefix` trait
- Added `derive` feature to enable the derive macro
- Added examples for using the derive macro
- Added namespaced attribute `#[entid(...)]` to avoid collisions with other crates

### Changed
- Updated documentation to include information about the derive macro
- Improved error messages for the derive macro

## [0.1.0] - 2024-03-03

### Added
- Initial release
- Support for UUID and ULID identifiers
- Type-safe entity IDs with prefixes
- Serde compatibility
- Comprehensive error handling 