# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-01-17

### Added
- Initial release
- Thread-safe static initialization with `StaticCell`
- Memory safety optimizations and compile-time assertions
- No-std support for embedded systems
- Comprehensive macro system including:
  - `static_cell!` for static initialization
  - `const_assert_size!` and `const_assert_align!` for memory layout verification
  - Formatting and printing utilities for no_std environments
  - Bitflags implementation
- Full test coverage
- Documentation with examples
