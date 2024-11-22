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
- 🧰 Comprehensive formatting utilities for no_std environments

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
noir_macros_core = "1.0.0"
```

## Quick Start

### Thread-Safe Static Initialization

```rust
use noir_macros_core::StaticCell;

// Using StaticCell with Default trait
static CONFIG: StaticCell<String> = StaticCell::new();

fn init_config() {
    // Thread-safe initialization
    if CONFIG.try_init(String::from("production")) {
        println!("Configuration initialized");
    }
    
    // Safe concurrent access
    if let Some(config) = CONFIG.get() {
        println!("Current config: {}", config);
    }
}
```

### Bit Flag Operations

```rust
use noir_macros_core::bitflags;

bitflags! {
    pub struct Permissions: u32 {
        const READ = 0b001;
        const WRITE = 0b010;
        const EXECUTE = 0b100;
    }
}

let mut perms = Permissions::READ | Permissions::WRITE;
assert!(perms.contains(Permissions::READ));
```

## Safety and Guarantees

- Thread-safe initialization through atomic operations
- Compile-time checks for memory layout and alignment
- Zero runtime overhead for most operations
- Safe concurrent access to static variables

## Contributing

We welcome contributions! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Credits

Created and maintained by Viicell and the Noir Framework Contributors.
