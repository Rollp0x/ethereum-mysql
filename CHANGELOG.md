# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.1] - 2025-07-08

### ✨ API Ergonomics & Macro Improvements
- **New conversion methods for SqlU256:** Added `as_u8`, `as_u16`, `as_u32`, `as_u64`, `as_u128` methods for ergonomic and safe conversion to primitive types, returning `Result` on overflow.
- **Bidirectional comparison:** Now supports direct comparison between `SqlU256` and all unsigned primitives (`u8`, `u16`, `u32`, `u64`, `u128`) in both directions (`SqlU256 == u32` and `u32 == SqlU256`).
- **Macro renaming:** Renamed macro `sqltopic!` to `sqlhash!` for clarity and consistency, as both topic and transaction hashes are 32-byte hashes.
- **Macro convenience:** All macros (`sqladdress!`, `sqlhash!`, `sqlu256!`) now support compile-time checked, ergonomic constant creation for Ethereum types.
- **Docs & comments:** Improved documentation and usage examples for all macros and new APIs.
- **Internal cleanup:** Removed unnecessary or duplicate trait implementations, clarified type boundaries, and improved code comments.

### 🛠️ Other
- **No Default for SqlFixedBytes<32>:** Confirmed that hash types do not implement `Default`, avoiding meaningless zero values for hashes.
- **Version bump:** Crate version updated to 3.0.1.

## [3.0.0] - 2025-06-30

### 🚨 Breaking Changes & Major Refactor
- **String-based storage only:** All binary mode support and related feature flags have been removed. The crate now exclusively supports string (hex) storage for all Ethereum types in SQLx-compatible databases.
- **Simplified API:** All wrapper types (`SqlAddress`, `SqlU256`, `SqlFixedBytes`, `SqlBytes`) now only support string-based storage and SQLx integration. No more binary column support or feature flag complexity.
- **Type aliases for hashes:** `SqlHash` and `SqlTopicHash` are now exported as type aliases for `SqlFixedBytes<32>`, making common 32-byte hash usage more ergonomic and explicit.
- **Documentation overhaul:** README and crate docs rewritten in English, with a clear focus on string-based storage, type safety, and API ergonomics. All Chinese content removed.
- **Test coverage:** Added robust unit and integration tests for all supported types and all three major backends (MySQL, PostgreSQL, SQLite), all using string storage.
- **Serde support:** Optional serde integration remains for all wrappers, behind the `serde` feature flag.
- **Versioning:** This release is not backward compatible with any previous version that used binary storage or feature flags for mode selection. Bump to 3.0.0 as a breaking change.

### ⚠️ Migration Notes
- **All users must migrate database columns to string (hex) types** (e.g. `VARCHAR`, `TEXT`) for all Ethereum types. See README for recommended column types.
- **Remove all feature flags related to binary mode** from your `Cargo.toml` and build scripts.
- **Update imports:** Use `SqlHash`/`SqlTopicHash` for 32-byte hashes and topics.
- **See README for updated usage examples and migration guidance.**

### ✨ Other Improvements
- **Cleaner, minimal codebase:** All code and documentation now focus on real-world, type-safe, ergonomic, and string-based Ethereum type storage for SQLx.
- **Better onboarding:** Usage examples and documentation are now more concise and practical for new users.

## [2.1.0] - 2025-06-28

### ✨ Major New Features
- **Dual database column type support**: Now supports both binary (BINARY/VARBINARY/BYTEA) and string (VARCHAR/CHAR/TEXT) column types for `SqlAddress` and `SqlU256`, controlled by feature flags.
- **New mutually exclusive features**:
  - `sqlx`/`sqlx_binary`: Enables binary column support (recommended for new projects)
  - `sqlx_str`: Enables string column support (for legacy/multi-language DBs)
  - **Compile error if both enabled**: These features are now mutually exclusive to prevent trait conflicts and schema ambiguity.
- **Feature combos**: Added `sqlx_full` and `sqlx_str_full` for convenient all-in-one feature enabling.
- **Comprehensive documentation**: Updated README and crate docs to clearly explain feature usage, recommended column types, and best practices for each mode.
- **Test coverage**: Added/updated integration tests for both binary and string modes across SQLite, MySQL, and PostgreSQL.
- **CI/automation**: Provided shell/CI scripts and guidance for testing all valid feature combinations.

### 🛠️ Improvements & Fixes
- **Error handling**: Improved error messages and trait gating for invalid feature combinations.
- **Examples**: All examples now work with both binary and string modes, with clear feature requirements in docs.
- **Internationalization**: All comments and test docs now in English for global accessibility.
- **Manifest cleanup**: Removed unused manifest keys and improved feature flag structure in `Cargo.toml`.

### ⚠️ Migration Notes
- **No breaking changes** for users who follow the new feature flag guidance.
- **Users must choose either binary or string mode** for database integration; do not enable both at once.
- **See README for migration and best practice details.**

## [2.0.0] - 2025-06-27

### Breaking Changes
- Major refactor: SqlU256 and related types now use the ruint-style implementation for improved performance and compatibility.
- **Database schema change:**
  - The recommended database column type for `SqlAddress` is now `BINARY(20)` (MySQL/SQLite), `BYTEA` (Postgres), or an equivalent binary type. This ensures correct roundtrip and storage of Ethereum addresses.
  - If you previously used `TEXT` or other string types, you must migrate your schema and data to binary format.
- This release is not backward compatible with previous versions that stored addresses as text.

### Other
- Updated documentation and examples to reflect the new binary storage format and ruint-based implementation.
- Minimum required Rust version and dependency versions unchanged.

## [1.6.1] - 2025-06-21

### 🔧 Dependencies

#### Alloy Version Optimization
- **Improved Compatibility**: Lowered minimum `alloy` version requirement from `1.0.7` to `1.0.3`
- **Better Ecosystem Integration**: Enhanced compatibility with projects using older alloy versions
- **Reduced Version Conflicts**: Minimized potential dependency resolution conflicts in user projects

## [1.6.0] - 2025-06-18

### 🚀 Major New Features

#### Comprehensive Comparison and Ordering Support
- **NEW**: `PartialOrd` and `Ord` trait implementations for both `SqlAddress` and `SqlU256`
- **Full Comparison Operations**: Support for `>`, `<`, `>=`, `<=` operators on both types
- **Collection Sorting**: Enable `.sort()` on `Vec<SqlU256>` and `Vec<SqlAddress>`
- **Min/Max Operations**: Built-in `.min()` and `.max()` methods for both types

#### DeFi and Blockchain-Optimized Features
- **UniswapV2 Compatible**: Perfect for token address ordering (`token0 < token1`)
- **Balance Comparisons**: Ideal for fund checks, transfer validations, and balance thresholds
- **Price Analysis**: Seamless price comparisons and ratio calculations
- **Gas Optimization**: Direct comparison for gas fees and cost analysis

#### Real-World Use Cases
- **DEX Development**: Automatic token pair ordering for consistent pair addresses
- **Security Checks**: Balance validation before transfers and operations
- **Portfolio Management**: Asset sorting and price range analysis
- **Database Queries**: Enhanced SQL compatibility with comparison-based filtering

### 📊 Examples and Documentation
- **NEW**: `comparison_demo.rs` - Comprehensive demonstration of all comparison features
- **NEW**: `uniswap_ordering.rs` - Real-world UniswapV2 token ordering scenarios
- **Enhanced README**: Added comparison examples for DeFi and blockchain use cases
- **Complete Test Coverage**: 5 new comprehensive tests covering all comparison scenarios

### ⚡ Performance and Compatibility
- **Zero Overhead**: Comparison operations delegate directly to underlying `alloy::primitives` types
- **Consistent Behavior**: Identical comparison logic to native `U256` and `Address` types
- **Full Interoperability**: Seamless sorting and comparison with mixed collection types

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

[1.6.0]: https://github.com/Rollp0x/ethereum-mysql/releases/tag/v1.6.0
[1.6.0]: https://github.com/Rollp0x/ethereum-mysql/releases/tag/v1.6.0
[1.5.0]: https://github.com/Rollp0x/ethereum-mysql/releases/tag/v1.5.0
[1.0.0]: https://github.com/Rollp0x/ethereum-mysql/releases/tag/v1.0.0