# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-06-09

### Added
- Initial stable release of ethereum-mysql
- `SqlAddress` wrapper for Ethereum addresses with SQLx integration
- Support for MySQL, PostgreSQL, and SQLite databases via SQLx
- `sqladdress!` macro for compile-time address creation and validation
- Pre-defined constants (`SqlAddress::ZERO`)
- Optional serde support for JSON serialization
- Comprehensive test suite with database integration tests
- Complete documentation with usage examples
- Feature flags for selective database support (`mysql`, `postgres`, `sqlite`, `serde`, `full`)

### Features
- Zero-cost abstractions wrapping `alloy::primitives::Address`
- Compile-time address validation and creation
- Seamless SQLx database integration with automatic type conversion
- Pure Rust implementation with no C dependencies
- Support for const contexts and static declarations

### Database Support
- **MySQL/MariaDB**: Store addresses as VARCHAR(42)
- **PostgreSQL**: Store addresses as VARCHAR(42) or TEXT
- **SQLite**: Store addresses as TEXT

### Documentation
- Comprehensive API documentation
- Usage examples for all major features
- Database schema examples
- Integration examples with real SQLx code

[1.0.0]: https://github.com/Rollp0x/ethereum-mysql/releases/tag/v1.0.0