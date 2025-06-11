# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.5.0] - 2025-06-10

### 🚀 Major New Features

#### Revolutionary Primitive Operations
- **NEW**: Direct arithmetic operations between `SqlU256` and Rust primitives
- Support for all primitive types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- Bidirectional operations: `value * 2` and `2 * value` both work seamlessly
- Intuitive syntax: `let fee = balance * 25 / 10000;` (no more verbose conversions!)
- Added comprehensive `primitive_ops.rs` module (~400 lines of optimized code)

#### Enhanced SqlU256 Capabilities
- **NEW**: `SqlU256::ZERO` constant for compile-time zero values
- Expanded arithmetic operations with better error handling
- Improved conversion traits for all Rust integer types

#### Hash Implementation & Collections Support
- **NEW**: `Hash` trait implementation for both `SqlAddress` and `SqlU256`
- **Perfect Hash Consistency**: Produces identical hash values as underlying `alloy::primitives` types
- **Collections Ready**: Full support for `HashMap` and `HashSet` operations
- **Zero Performance Overhead**: Hash performance equal to or better than underlying types
- **Interoperability**: Seamless conversion between wrapped and unwrapped types in collections

### 🎯 Code Optimizations

#### Macro-Based Implementations
- **Optimized**: `convert.rs` - 36% reduction in code size (259→166 lines)
- **Optimized**: `operation.rs` - 50% reduction in code size (220→110 lines)
- Introduced efficient macros: `impl_from_unsigned!`, `impl_try_from_signed!`, `impl_binary_op!`
- Better maintainability and consistency across trait implementations

#### API Improvements
- **Removed**: Confusing type aliases from public API for better clarity
- **Enhanced**: Error handling and edge case coverage
- **Improved**: Documentation with real-world examples

### 📚 Documentation & Examples

#### Comprehensive README Expansion
- **NEW**: "Key Advantages" section with comparison tables
- **NEW**: Web3 API integration examples showing automatic validation
- **NEW**: Real-world impact examples and performance comparisons
- **Enhanced**: Usage examples with primitive operations

#### New Example Files
- **NEW**: `primitive_ops_demo.rs` - Comprehensive primitive operations showcase
- **NEW**: `hash_consistency_demo.rs` - Demonstrates hash consistency between Sql* and alloy types
- **Enhanced**: `basic_usage.rs`, `const_demo.rs`, and `zero_constant.rs`
- **Improved**: All examples with English comments and better explanations

#### Documentation Translation
- **Fixed**: Doctest compilation errors in `lib.rs`
- **Translated**: All Chinese comments to English for international accessibility
- **Updated**: API documentation with clearer examples and use cases

### 🧪 Testing Improvements

#### Enhanced Test Coverage
- **NEW**: Dedicated serde integration tests (`sql_u256_serde_integration.rs`)
- **NEW**: Hash consistency tests for both `SqlAddress` and `SqlU256`
- **NEW**: Collections compatibility tests (HashMap/HashSet operations)
- **Optimized**: Unit tests in `sql_u256.rs` (focused on 10 core test cases)
- **Enhanced**: `const_test.rs` with `SqlU256::ZERO` constant testing
- **Total**: 68 tests passing (49 unit + 6 const + 2 integration + 11 doc tests)

#### Test Organization
- **Separated**: Unit tests from integration tests for better organization
- **Added**: Edge case testing for primitive operations
- **Improved**: Test naming and documentation

### 🔧 Technical Improvements

#### Performance Optimizations
- Zero-cost abstractions for primitive operations
- Compile-time evaluation of constants
- Reduced binary size through macro consolidation

#### Code Quality
- **Improved**: Error messages and debugging information
- **Enhanced**: Type safety with better conversion handling
- **Standardized**: Code formatting and documentation style

### 📦 Package Updates
- Updated version to 1.5.0
- Maintained full backward compatibility
- All existing code continues to work without changes

### 🌟 Key Benefits for Users
- **Intuitive Code**: Write `balance * 25 / 10000` instead of verbose conversions
- **Better Performance**: Optimized implementations with reduced overhead
- **Enhanced Developer Experience**: Clearer documentation and examples
- **Bulletproof APIs**: Automatic validation in Web3 applications
- **Collections Ready**: Use `SqlAddress`/`SqlU256` directly in `HashMap` and `HashSet`
- **Perfect Interoperability**: Hash-consistent with underlying alloy types
- **Zero Breaking Changes**: Seamless upgrade from 1.0.0

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

[1.5.0]: https://github.com/Rollp0x/ethereum-mysql/releases/tag/v1.5.0
[1.0.0]: https://github.com/Rollp0x/ethereum-mysql/releases/tag/v1.0.0