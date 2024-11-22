# noir_macros_core

[![Crates.io](https://img.shields.io/crates/v/noir_macros_core.svg)](https://crates.io/crates/noir_macros_core)
[![Documentation](https://docs.rs/noir_macros_core/badge.svg)](https://docs.rs/noir_macros_core)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Essential procedural macros and utilities for no_std Rust development, part of the Noir Framework. This crate provides zero-dependency, thread-safe utilities designed specifically for embedded and no_std environments.

## Features

- 🚀 Zero external dependencies for minimal footprint
- 💻 Full no_std compatibility for embedded systems
- 🛡️ Thread-safe static initialization with `StaticCell`
- ⚡ Compile-time memory layout assertions
- 🔒 Strong safety guarantees through Rust's type system
- 📦 Robust bit flag operations and manipulation
- 📝 Advanced formatting utilities with dynamic buffer management
- 🔧 Memory-efficient string handling for resource-constrained systems

## [1.1.1] - 2024-11-22

### Fixed
- Printing issues in `print!` macro
- Buffer overflow issue in `format!` macro
- Fixed buffer overflow issues in `print!` macro
- Improved buffer growth strategy for better memory management
- Restored proper thread safety for `Buffer` type
- Fixed synchronization issues in static buffer handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
noir_macros_core = "1.1.2"
```

## Usage Examples

### String Formatting

```rust
use noir_macros_core::format;

// Basic formatting
let name = "World";
let greeting = format!("Hello, {}!", name);

// Complex formatting with multiple arguments
let count = 42;
let value = 3.14;
let result = format!("Count: {}, Value: {:.2}", count, value);
```

### Static Initialization

```rust
use noir_macros_core::static_cell;

// Thread-safe static configuration
static_cell!(CONFIG, &str);

fn init() {
    if CONFIG.try_init("production") {
        // Successfully initialized
    }
    
    if let Some(config) = CONFIG.get() {
        println!("Current config: {}", config);
    }
}
```

### Memory Layout Verification

```rust
use noir_macros_core::{const_assert_size, const_assert_align};

#[repr(C)]
struct CriticalData {
    flags: u32,    // 4 bytes
    active: bool,  // 1 byte
    _pad: [u8; 3], // 3 bytes padding
}

const_assert_size!(CriticalData, 8);   // Ensure total size
const_assert_align!(CriticalData, 4);  // Ensure alignment
```

## Performance Considerations

- Zero-cost abstractions where possible
- Efficient memory usage with dynamic buffer allocation
- Thread-safe operations with minimal overhead
- Compile-time validations for optimal runtime performance

## Contributing

We welcome contributions! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
