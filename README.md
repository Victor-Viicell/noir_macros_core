# noir_macros_core

[![Crates.io](https://img.shields.io/crates/v/noir_macros_core.svg)](https://crates.io/crates/noir_macros_core)
[![Documentation](https://docs.rs/noir_macros_core/badge.svg)](https://docs.rs/noir_macros_core)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Essential procedural macros and utilities for no_std Rust development, part of the Noir Framework.

## Features

- 🚀 Zero external dependencies
- 💻 100% no_std compatible
- 🛡️ Thread-safe static initialization with `Default` trait support
- ⚡ Compile-time memory layout assertions
- 🔒 Strong safety guarantees through Rust's type system
- 📦 Robust bit flag operations
- 🧰 Comprehensive formatting utilities for no_std environments

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
noir_macros_core = "1.0.0"
```

## Key Features

### Thread-Safe Static Initialization

```rust
use noir_macros_core::StaticCell;

// Using the Default trait (new in 1.0.0)
let cell: StaticCell<i32> = Default::default();
assert!(cell.try_init(42));

// Or using the static_cell macro
static_cell!(CONFIG: &str);

fn init() {
    // Only the first thread will succeed in initialization
    if CONFIG.try_init("production") {
        // Successfully initialized
    }
    
    // All threads can safely read the value
    if let Some(config) = CONFIG.get() {
        println!("Current config: {}", config);
    }
}
```

### Memory Safety and Optimization

```rust
use noir_macros_core::{const_assert_size, const_assert_align};

#[repr(C)]
struct CriticalData {
    flags: u32,    // 4 bytes
    active: bool,  // 1 byte
    _pad: [u8; 3], // 3 bytes padding
}

// Verify memory layout at compile-time
const_assert_size!(CriticalData, 8);   // Ensure total size
const_assert_align!(CriticalData, 4);  // Ensure alignment
```

### Type-Safe Bit Flags

```rust
use noir_macros_core::bitflags;

bitflags! {
    /// Process capabilities
    pub struct Capabilities: u32 {
        const READ_FILES  = 0b0001;
        const WRITE_FILES = 0b0010;
        const NETWORK     = 0b0100;
    }
}

// Type system ensures flags can only be combined safely
let caps = Capabilities::READ_FILES | Capabilities::WRITE_FILES;
assert!(caps.contains(Capabilities::READ_FILES));
```

## Implementation Details

### Memory Ordering

The crate carefully chooses memory orderings for atomic operations to ensure thread safety:

```rust
use core::sync::atomic::Ordering;

// Reading uses Acquire ordering to sync with initialization
let value = atomic.load(Ordering::Acquire);

// Initialization uses AcqRel for bidirectional synchronization
let result = atomic.compare_exchange(
    false, true,
    Ordering::AcqRel,
    Ordering::Relaxed
);
```

### Safety Guarantees

- Thread-safety through atomic operations
- Memory safety via proper alignment and access patterns
- Type safety using Rust's type system
- Compile-time guarantees where possible
- Zero unsafe code in public APIs

### No-std Support

Designed for environments without the standard library:
- Embedded systems
- Operating system development
- Resource-constrained platforms

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
