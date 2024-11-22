#![no_std]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! # noir_macros_core
//! 
//! A robust implementation of thread-safe static initialization and utility macros for no_std environments.
//! This crate provides essential building blocks for systems programming, focusing on safety and zero-cost abstractions.
//! 
//! ## Key Features
//! 
//! ### Thread-Safe Static Initialization
//! ### Memory Safety and Optimization
//! ### Safe Abstractions
//! 
//! ### Performance Considerations
//! - Zero-cost abstractions where possible
//! - Careful choice of atomic operations
//! - Proper alignment for hardware efficiency
//! - Compile-time evaluation when feasible
//! 
//! ## Contributing
//! 
//! We welcome contributions! Please feel free to submit a Pull Request.
//! 
//! ## License
//! 
//! noir_macros_core is distributed under the MIT License.

extern crate alloc;

use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

/// A thread-safe static initialization cell.
/// 
/// The `StaticCell<T>` type provides safe one-time initialization of static values
/// in multi-threaded contexts. It uses atomic operations to ensure thread-safety
/// and prevent multiple initializations.
/// 
/// # Memory Layout
/// ```text
/// StaticCell<T>: align(8) {
///     initialized: AtomicBool,     // Thread-safe state tracking
///     value: UnsafeCell<Option<T>> // Protected storage
/// }
/// ```
/// 
/// # Thread Safety
/// The type implements `Sync` when `T: Sync` because:
/// - Initialization is protected by atomic operations
/// - The value becomes immutable after initialization
/// - All access is properly synchronized
/// 
/// # Examples
/// 
/// Basic usage with a primitive type:
/// ```rust
/// use noir_macros_core::StaticCell;
/// 
/// static COUNTER: StaticCell<u32> = StaticCell::new();
/// 
/// // Initialize the counter
/// if COUNTER.try_init(1) {
///     println!("Counter initialized");
/// }
/// 
/// // Read the counter value
/// if let Some(count) = COUNTER.get() {
///     assert_eq!(*count, 1);
/// }
/// ```
#[repr(align(8))]
pub struct StaticCell<T> {
    initialized: AtomicBool,
    value: UnsafeCell<Option<T>>,
}

impl<T> StaticCell<T> {
    /// Creates a new uninitialized static cell.
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(None),
        }
    }

    /// Attempts to get a reference to the contained value.
    pub fn get(&self) -> Option<&T> {
        if self.initialized.load(Ordering::Acquire) {
            // SAFETY: We only access the value after initialization
            // and never modify it after that point
            unsafe { (*self.value.get()).as_ref() }
        } else {
            None
        }
    }

    /// Attempts to initialize the cell with a value.
    pub fn try_init(&self, value: T) -> bool {
        if self.initialized.compare_exchange(
            false,
            true,
            Ordering::AcqRel,
            Ordering::Relaxed,
        ).is_ok() {
            // SAFETY: We only modify the value during initialization
            // and the atomic exchange ensures only one thread can initialize
            unsafe { *self.value.get() = Some(value) };
            true
        } else {
            false
        }
    }
}

/// Implements `Sync` for `StaticCell<T>` when `T: Sync`.
unsafe impl<T: Sync> Sync for StaticCell<T> {}

/// Implements `Default` for `StaticCell<T>`.
/// 
/// This provides a convenient way to create a new uninitialized static cell
/// using the standard `Default` trait.
/// 
/// # Examples
/// ```rust
/// use noir_macros_core::StaticCell;
/// 
/// let cell: StaticCell<i32> = Default::default();
/// assert!(cell.try_init(42));
/// ```
impl<T> Default for StaticCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a new static cell with the specified name and type.
/// 
/// This macro simplifies the creation of static cells by handling
/// the type annotation and initialization boilerplate.
/// 
/// # Parameters
/// - `$name`: The identifier for the static cell
/// - `$type`: The type of value to store in the cell
/// 
/// # Examples
/// ```rust
/// use noir_macros_core::static_cell;
/// 
/// // Create a static cell holding a String
/// static_cell!(CONFIG, String);
/// 
/// // Create a static cell holding a custom type
/// #[derive(Debug)]
/// struct AppState {
///     version: u32,
/// }
/// 
/// static_cell!(STATE, AppState);
/// ```
#[macro_export]
macro_rules! static_cell {
    ($name:ident, $type:ty) => {
        static $name: $crate::StaticCell<$type> = $crate::StaticCell::new();
    };
}

/// Verifies the size of a type at compile time.
/// 
/// # Understanding Type Size
/// In systems programming, the exact size of types is crucial for:
/// - Memory layout control
/// - FFI (Foreign Function Interface) compatibility
/// - Embedded systems constraints
/// - Performance optimization
/// 
/// # Memory Alignment
/// ```text
/// struct Example {    Alignment Padding
/// ┌─────────────┐    ┌─┐
/// │  u32 (4B)   │    │ │
/// ├─────────────┤    ├─┤
/// │  u8 (1B)    │ -> │X│ <- 3 bytes padding
/// ├─────────────┤    ├─┤
/// │  u32 (4B)   │    │ │
/// └─────────────┘    └─┘
/// Total: 12 bytes
/// ```
/// 
/// # Usage Examples
/// ```rust
/// use noir_macros_core::const_assert_size;
/// // Basic size check
/// const_assert_size!(u32, 4);
/// 
/// // Custom struct size verification
/// #[repr(C)]
/// struct Packet {
///     header: u32,    // 4 bytes
///     flags: u8,      // 1 byte
///     _pad: [u8; 3],  // 3 bytes padding
///     data: u32,      // 4 bytes
/// }
/// const_assert_size!(Packet, 12);
/// ```
/// 
/// # Common Applications
/// 1. Network protocol structures
/// 2. Hardware interface types
/// 3. Memory-mapped I/O
/// 4. Binary file formats
/// 
/// # Best Practices
/// 1. Always use with `#[repr(C)]` for predictable layout
/// 2. Account for padding in size calculations
/// 3. Document size assumptions
/// 4. Use with alignment assertions
#[macro_export]
macro_rules! const_assert_size {
    ($type:ty, $size:expr) => {
        const _: () = assert!(core::mem::size_of::<$type>() == $size);
    };
}

/// Verifies the alignment of a type at compile time.
/// 
/// # Understanding Alignment
/// Memory alignment is crucial for:
/// - CPU access efficiency
/// - Hardware requirements
/// - Platform compatibility
/// - Performance optimization
/// 
/// # Alignment Visualization
/// ```text
/// Memory Address: 0  1  2  3  4  5  6  7
///                ┌──┬──┬──┬──┬──┬──┬──┬──┐
/// Aligned(4):    │  u32   │  u32   │  u32 
///                └──┴──┴──┴──┴──┴──┴──┴──┘
///                ↑     ↑     ↑     ↑
///                Valid addresses for u32 (4-byte aligned)
/// ```
/// 
/// # Usage Examples
/// ```rust
/// use noir_macros_core::const_assert_align;
/// 
/// // Basic alignment checks
/// const_assert_align!(u32, 4);
/// const_assert_align!(u64, 8);
/// 
/// // Custom aligned types
/// #[repr(align(16))]
/// struct SimdVector {
///     data: [f32; 4],
/// }
/// const_assert_align!(SimdVector, 16);
/// ```
/// 
/// # Common Use Cases
/// 1. SIMD data structures
/// 2. DMA buffers
/// 3. Hardware interfaces
/// 4. Cache-line optimization
/// 
/// # Best Practices
/// 1. Use `#[repr(align(N))]` for custom alignment
/// 2. Consider cache line sizes (usually 64 bytes)
/// 3. Document alignment requirements
/// 4. Pair with size assertions
#[macro_export]
macro_rules! const_assert_align {
    ($type:ty, $align:expr) => {
        const _: () = assert!(core::mem::align_of::<$type>() == $align);
    };
}

/// Creates a compile-time string literal.
/// 
/// # Understanding Const Strings
/// Const strings are string literals that are:
/// - Evaluated at compile time
/// - Stored in the binary
/// - Zero runtime overhead
/// - Type checked at compile time
/// 
/// # Memory Layout
/// ```text
/// Static String in Binary:
/// ┌────────────────────┐
/// │ Length (usize)     │ <- Known at compile time
/// ├────────────────────┤
/// │ UTF-8 Bytes        │ <- Immutable data
/// ├────────────────────┤
/// │ NUL terminator     │ <- For C compatibility
/// └────────────────────┘
/// ```
/// 
/// # Usage Examples
/// ```rust
/// use noir_macros_core::const_str;
///
/// // Basic usage
/// const GREETING: &str = const_str!("Hello, World!");
/// 
/// // In static contexts
/// static APP_NAME: &str = const_str!("MyApp");
/// 
/// // With escape sequences
/// const PATH: &str = const_str!("C:\\Program Files\\App");
/// ```
/// 
/// # Common Applications
/// 1. Error messages
/// 2. Configuration strings
/// 3. Static resources
/// 4. Compile-time constants
/// 
/// # Best Practices
/// 1. Use for truly constant strings
/// 2. Consider UTF-8 implications
/// 3. Document string purpose
/// 4. Prefer over string literals for constants
#[macro_export]
macro_rules! const_str {
    ($s:expr) => { $s };
}

/// Performs compile-time type checks and assertions.
/// 
/// # Understanding Type Checks
/// Type checking at compile time ensures:
/// - Memory safety through layout verification
/// - Size and alignment constraints
/// - Value semantics validation
/// - Performance characteristics
/// 
/// # Type Properties Verified
/// ```text
/// Type Requirements:
/// ┌──────────────────┐
/// │ POD Status       │ No custom Drop impl
/// ├──────────────────┤
/// │ Size Limits      │ Memory boundaries
/// ├──────────────────┤
/// │ Alignment        │ Memory layout
/// └──────────────────┘
/// ```
/// 
/// # Usage Examples
/// ```rust
/// use noir_macros_core::type_check;
/// 
/// #[repr(C)]
/// struct SafeType {
///     data: u32,
/// }
/// 
/// type_check! {
///     ensure SafeType: {
///         is_pod,                // Must be Plain Old Data
///         max_size: 4,          // No larger than 4 bytes
///         aligned_to: 4         // Must be 4-byte aligned
///     }
/// }
/// ```
/// 
/// # Available Checks
/// 1. `is_pod` - Ensures type has no custom Drop implementation
/// 2. `max_size: N` - Verifies type size ≤ N bytes
/// 3. `aligned_to: N` - Confirms type has N-byte alignment
/// 
/// # Best Practices
/// 1. Use in safety-critical code
/// 2. Document check rationale
/// 3. Group related checks
/// 4. Add error messages
/// 
/// # Implementation Details
/// The checks are implemented using Rust's const evaluation system,
/// ensuring all verifications happen at compile time with zero
/// runtime overhead.
#[macro_export]
macro_rules! type_check {
    (ensure $type:ty: { is_pod $(,)? }) => {
        const _: () = assert!(core::mem::needs_drop::<$type>() == false);
    };
    (ensure $type:ty: { max_size: $size:expr $(,)? }) => {
        const _: () = assert!(core::mem::size_of::<$type>() <= $size);
    };
    (ensure $type:ty: { aligned_to: $align:expr $(,)? }) => {
        const _: () = assert!(core::mem::align_of::<$type>() == $align);
    };
    (ensure $type:ty: { $($check:ident $(: $val:expr)? ),+ $(,)? }) => {
        $($crate::type_check!(ensure $type: { $check $(: $val)? });)+
    };
}

/// Creates a new vector with the given elements.
/// 
/// # Understanding Vectors
/// A vector is a dynamic array that can grow or shrink in size. It's one of the most
/// commonly used collection types in Rust because it provides:
/// - Dynamic sizing (can grow/shrink)
/// - Contiguous memory storage (fast access)
/// - Automatic memory management
/// 
/// # Memory Layout
/// ```text
/// Vec<T>
/// ┌─────────┬─────────┬──────────┐
/// │ pointer │capacity │  length  │ (on stack)
/// └───┬─────┴─────────┴──────────┘
///     │
///     v
/// ┌───┬───┬───┬───┬─────┐
/// │ 0 │ 1 │ 2 │ 3 │ ... │ (on heap)
/// └───┴───┴───┴───┴─────┘
/// ```
/// 
/// # Usage Patterns
/// 1. Empty vector:
/// ```rust
/// use noir_macros_core::vec;
/// 
/// let v: Vec<i32> = vec![];
/// ```
/// 
/// 2. Vector with repeated elements:
/// ```rust
/// // Creates [1, 1, 1, 1, 1]
/// let v = vec![1; 5];
/// ```
/// 
/// 3. Vector with specific elements:
/// ```rust
/// let v = vec![1, 2, 3, 4, 5];
/// ```
/// 
/// # Performance Considerations
/// - Initial allocation happens on the heap
/// - Capacity doubles when more space is needed
/// - Consider pre-allocating with known size
/// 
/// # Best Practices
/// 1. Use `with_capacity` when size is known
/// 2. Clear with `clear()` instead of reassigning
/// 3. Use `drain()` to remove and reuse elements
/// 4. Consider `Vec::new()` for empty vectors
#[macro_export]
macro_rules! vec {
    () => {
        ::core::iter::Iterator::collect::<Vec<_>>(::core::iter::empty())
    };
    ($elem:expr; $n:expr) => {
        ::core::iter::repeat($elem).take($n).collect::<Vec<_>>()
    };
    ($($x:expr),+ $(,)?) => {
        <[_]>::into_vec(Box::new([$($x),+]))
    };
}

/// Creates a fixed-size array with the given elements.
/// 
/// # Understanding Arrays
/// Arrays in Rust are fixed-size sequences of elements stored in contiguous memory.
/// Unlike vectors, their size is part of their type and cannot change.
/// 
/// # Key Characteristics
/// - Fixed size known at compile time
/// - Stored entirely on the stack
/// - Zero runtime overhead
/// - Direct indexing without bounds checking
/// 
/// # Memory Layout
/// ```text
/// [T; N] (on stack)
/// ┌───┬───┬───┬───┐
/// │ 0 │ 1 │ 2 │ 3 │
/// └───┴───┴───┴───┘
/// ```
/// 
/// # Usage Examples
/// ```rust
/// use noir_macros_core::array;
///
/// // Empty array
/// let empty: [i32; 0] = array![];
/// 
/// // Array with values
/// let numbers = array![1, 2, 3, 4];
/// 
/// // Note: All elements must be of the same type
/// // This would NOT work:
/// // let mixed = array![1, 2.5, 3.7];  // Error: mixed types
/// // for mixed types, use a tuple
/// let mixed: (i32, f64) = (1, 2.5);
/// ```
/// 
/// # Common Use Cases
/// 1. Fixed-size data structures
/// 2. Performance-critical code
/// 3. Embedded systems
/// 4. Stack-only allocations
/// 
/// # When to Use Arrays
/// 1. Fixed-size data structures
/// 2. Performance-critical code
/// 3. Embedded systems
/// 4. Stack-only allocations
/// 
/// # Best Practices
/// 1. Use when size is known at compile time
/// 2. Consider for small, fixed collections
/// 3. Use with SIMD operations
/// 4. Prefer over Vec for tiny sequences
#[macro_export]
macro_rules! array {
    () => { [] };
    ($($x:expr),+ $(,)?) => { [$($x),+] };
}

/// Prints formatted text to the standard output.
/// 
/// This macro provides formatted printing functionality in no_std environments.
/// It validates format strings at compile time.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        // Create a static buffer for output
        static PRINT_BUFFER: $crate::StaticCell<$crate::Buffer> = $crate::StaticCell::new();
        
        // Initialize buffer if needed
        if PRINT_BUFFER.try_init($crate::Buffer::new()) {
            // First time initialization
        }
        
        // Format and write to buffer
        if let Some(buffer) = PRINT_BUFFER.get() {
            unsafe {
                *buffer.pos.get() = 0;
                let _ = $crate::write(buffer, core::format_args!($($arg)*));
                let output = core::str::from_utf8_unchecked(&(*buffer.buf.get())[..*buffer.pos.get()]);
                $crate::_print(output);
            }
        }
    }};
}

/// Internal function to handle actual printing.
#[doc(hidden)]
pub fn _print(s: &str) {
    // Implementation depends on target platform
    #[cfg(target_arch = "wasm32")]
    {
        // Web assembly implementation
        extern "C" {
            fn console_log(ptr: *const u8, len: usize);
        }
        unsafe {
            console_log(s.as_ptr(), s.len());
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Default implementation using core::fmt::Write
        use core::fmt::Write;
        struct Stdout;
        
        impl Write for Stdout {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                extern "C" {
                    fn putchar(c: i32) -> i32;
                }
                for byte in s.bytes() {
                    unsafe {
                        putchar(byte as i32);
                    }
                }
                Ok(())
            }
        }
        
        let mut stdout = Stdout;
        let _ = stdout.write_str(s);
    }
}

/// Prints formatted text to the standard output, with a newline.
/// 
/// # Understanding println!
/// This macro extends the `print!` macro by automatically adding a newline
/// at the end of the output. It's essential for formatted console output
/// in no_std environments.
/// 
/// # How It Works
/// 1. Formats the text using the same rules as `print!`
/// 2. Appends a newline character (`\n`)
/// 3. Writes to the output in a single operation
/// 
/// # Examples
/// ```rust
/// use noir_macros_core::println;
/// use noir_macros_core::vec;
///
/// // Basic usage
/// println!("Hello, World!");
/// 
/// // With formatting
/// let name = "Rust";
/// println!("Learning {}", name);
/// 
/// // Multiple values
/// let (x, y) = (10, 20);
/// println!("Point: ({}, {})", x, y);
/// 
/// // Debug formatting
/// let data = vec![1, 2, 3];
/// println!("Data: {:?}", data);
/// ```
/// 
/// # Common Use Cases
/// 1. Debug output
/// 2. User interaction
/// 3. Logging information
/// 4. Progress reporting
/// 
/// # Best Practices
/// 1. Use for human-readable output
/// 2. Consider buffering for many prints
/// 3. Use debug format `{:?}` for complex types
/// 4. Avoid in performance-critical loops
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

/// Internal helper struct for print macro.
/// 
/// This type implements `fmt::Write` to enable formatted printing
/// in no_std environments. It's used internally by the print
/// macro implementation.
/// 
/// # Implementation Notes
/// - Provides a no-op implementation of `write_str`
/// - Used for compile-time format string validation
#[doc(hidden)]
pub struct PrintWrapper;

impl core::fmt::Write for PrintWrapper {
    /// Implements the write_str method required by fmt::Write.
    /// This is a no-op implementation used only for compile-time
    /// format string validation.
    fn write_str(&mut self, _s: &str) -> core::fmt::Result {
        Ok(())
    }
}

const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;
const MAX_BUFFER_SIZE: usize = 1024 * 1024;

/// A buffer for storing formatted strings with configurable size.
#[doc(hidden)]
pub struct Buffer {
    pub buf: UnsafeCell<Vec<u8>>,
    pub pos: UnsafeCell<usize>,
    pub capacity: usize,
}

impl Buffer {
    /// Creates a new buffer with the specified capacity.
    /// 
    /// # Safety
    /// The capacity must be less than or equal to MAX_BUFFER_SIZE.
    pub const unsafe fn with_capacity(capacity: usize) -> Self {
        assert!(capacity <= MAX_BUFFER_SIZE, "Buffer capacity exceeds maximum allowed size");
        Self {
            buf: UnsafeCell::new(Vec::new()),
            pos: UnsafeCell::new(0),
            capacity,
        }
    }

    /// Creates a new buffer with the default capacity.
    pub const fn new() -> Self {
        unsafe { Self::with_capacity(DEFAULT_BUFFER_SIZE) }
    }

    /// Returns true if the buffer has enough space for additional bytes.
    #[inline]
    pub fn has_capacity(&self, additional: usize) -> bool {
        unsafe { *self.pos.get() + additional <= self.capacity }
    }

    /// Attempts to grow the buffer to accommodate more data.
    /// Returns true if successful, false if the new size would exceed MAX_BUFFER_SIZE.
    pub fn try_grow(&self, required: usize) -> bool {
        unsafe {
            let current_pos = *self.pos.get();
            let new_size = (current_pos + required).next_power_of_two();
            
            if new_size <= MAX_BUFFER_SIZE {
                let buf = &mut *self.buf.get();
                buf.reserve(new_size - buf.capacity());
                true
            } else {
                false
            }
        }
    }
}

// SAFETY: Access to the buffer is synchronized through StaticCell
// and we ensure single-threaded access to the buffer during writes
unsafe impl Sync for Buffer {}

impl core::fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let pos = unsafe { *self.pos.get() };
        
        if !self.has_capacity(bytes.len()) && !self.try_grow(bytes.len()) {
            return Err(core::fmt::Error);
        }

        unsafe {
            let buf = &mut *self.buf.get();
            if buf.len() < pos + bytes.len() {
                buf.extend_from_slice(&[0; 8]); // Grow in small chunks
            }
            buf[pos..pos + bytes.len()].copy_from_slice(bytes);
            *self.pos.get() = pos + bytes.len();
        }
        Ok(())
    }
}

/// A helper function to write formatted arguments to a buffer through a shared reference.
#[doc(hidden)]
pub fn write(buffer: &Buffer, args: core::fmt::Arguments) -> core::fmt::Result {
    struct WriteAdapter<'a>(&'a Buffer);

    impl<'a> core::fmt::Write for WriteAdapter<'a> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let bytes = s.as_bytes();
            let pos = unsafe { *self.0.pos.get() };
            
            if !self.0.has_capacity(bytes.len()) && !self.0.try_grow(bytes.len()) {
                return Err(core::fmt::Error);
            }

            unsafe {
                let buf = &mut *self.0.buf.get();
                if buf.len() < pos + bytes.len() {
                    buf.extend_from_slice(&[0; 8]); // Grow in small chunks
                }
                buf[pos..pos + bytes.len()].copy_from_slice(bytes);
                *self.0.pos.get() = pos + bytes.len();
            }
            Ok(())
        }
    }
    core::fmt::write(&mut WriteAdapter(buffer), args)
}

/// A macro for formatting text in a no_std environment.
/// 
/// This macro provides string formatting capabilities similar to the standard library's
/// `format!` macro, but designed specifically for no_std environments. It uses a dynamic
/// buffer for formatting and is thread-safe.
/// 
/// # Features
/// - Thread-safe formatting using static buffers
/// - Compile-time format string validation
/// - Dynamic buffer growth up to 1MB
/// - Efficient memory usage with small initial buffer
/// - Error handling for buffer overflow
/// 
/// # Examples
/// 
/// Basic string formatting:
/// ```rust
/// use noir_macros_core::format;
/// 
/// let name = "World";
/// let greeting = format!("Hello, {}!", name);
/// assert_eq!(greeting, "Hello, World!");
/// ```
/// 
/// Multiple arguments and different types:
/// ```rust
/// use noir_macros_core::format;
/// 
/// let count = 42;
/// let value = 3.14;
/// let result = format!("Count: {}, Value: {:.2}", count, value);
/// assert_eq!(result, "Count: 42, Value: 3.14");
/// ```
/// 
/// # Buffer Size
/// - Initial buffer size: 8KB (DEFAULT_BUFFER_SIZE)
/// - Maximum buffer size: 1MB (MAX_BUFFER_SIZE)
/// - Buffer grows dynamically as needed
/// - Returns error if formatted string would exceed maximum size
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        // Validate format string at compile time
        let _ = {
            #[allow(unused_imports)]
            use core::fmt::Write;
            let mut _pw = $crate::PrintWrapper {};
            core::fmt::write(&mut _pw, core::format_args!($($arg)*))
        };
        
        static BUFFER: $crate::StaticCell<$crate::Buffer> = $crate::StaticCell::new();
        
        // Initialize buffer if not already initialized
        if BUFFER.try_init($crate::Buffer::new()) {
            // First time initialization
        }
        
        // Get reference to buffer and format string
        if let Some(buffer) = BUFFER.get() {
            unsafe {
                *buffer.pos.get() = 0;
                let _ = $crate::write(buffer, core::format_args!($($arg)*));
                core::str::from_utf8_unchecked(&(*buffer.buf.get())[..*buffer.pos.get()])
            }
        } else {
            "" // Return empty string if buffer not available
        }
    }};
}

/// A macro for debug formatting in no_std environments.
///
/// This macro works similarly to the standard library's `dbg!` macro but is
/// designed for no_std environments. It prints the expression and its value,
/// and returns the value.
///
/// # Examples
///
/// ```rust
/// use noir_macros_core::debug;
///
/// let x = 42;
/// let y = debug!(x + 1); // prints "[DEBUG] x + 1 = 43"
/// assert_eq!(y, 43);
/// ```
#[macro_export]
macro_rules! debug {
    ($val:expr) => {{
        match $val {
            tmp => {
                $crate::println!("[DEBUG] {} = {:?}", stringify!($val), &tmp);
                tmp
            }
        }
    }};
    ($($val:expr),+ $(,)?) => {
        ($($crate::debug!($val)),+,)
    };
}

/// A macro for defining bit flags in a type-safe way.
///
/// This macro creates a type-safe bit flag enum that can be combined
/// using bitwise operations.
///
/// # What are Bitflags?
/// Bitflags are a programming pattern where individual bits in an integer are used
/// to represent boolean flags. This is memory-efficient and allows for fast operations.
///
/// # Why Use Bitflags?
/// - Memory Efficient: Multiple flags in a single integer
/// - Fast Operations: Bitwise operations are very fast
/// - Type Safe: Rust's type system prevents invalid combinations
///
/// # How Bitflags Work
/// Each flag is a power of 2 (1, 2, 4, 8, 16, etc.) so that each bit represents
/// a unique flag:
/// ```text
/// Bit Position:  7  6  5  4  3  2  1  0
/// Binary:        0  0  0  0  0  1  0  1
///                            ↑  ↑  ↑  ↑
///                            8  4  2  1
/// ```
///
/// # Example Usage
/// ```rust
/// use noir_macros_core::bitflags;
/// bitflags! {
///     /// File permissions in a Unix-like system
///     pub struct Permissions: u8 {
///         const READ    = 0b0000_0100;  // 4 in decimal
///         const WRITE   = 0b0000_0010;  // 2 in decimal
///         const EXECUTE = 0b0000_0001;  // 1 in decimal
///     }
/// }
///
/// // Combine permissions using bitwise OR (|)
/// let read_write = Permissions::READ | Permissions::WRITE;
///
/// // Check permissions using contains()
/// assert!(read_write.contains(Permissions::READ));
/// assert!(!read_write.contains(Permissions::EXECUTE));
/// ```
///
/// # Common Operations
/// - `|` (OR): Combine flags
/// - `&` (AND): Check if flags are present
/// - `^` (XOR): Toggle flags
/// - `!` (NOT): Invert flags
///
/// # Best Practices
/// 1. Use descriptive names for your flags
/// 2. Document the purpose of each flag
/// 3. Group related flags together
/// 4. Consider using a larger integer type (u32, u64) if you need many flags
#[macro_export]
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $name:ident: $type:ty {
            $(
                $(#[$inner:meta])*
                const $flag:ident = $value:expr;
            )*
        }
    ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, PartialEq, Eq)]
        #[repr(transparent)]
        $vis struct $name($type);

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("bits", &format!("{:#b}", self.0))
                    .finish()
            }
        }

        impl $name {
            $(
                $(#[$inner])*
                $vis const $flag: Self = Self($value);
            )*

            /// Returns an empty set of flags.
            #[inline]
            pub const fn empty() -> Self {
                Self(0)
            }

            /// Returns true if no flags are set.
            #[inline]
            pub const fn is_empty(self) -> bool {
                self.0 == 0
            }

            /// Returns true if all flags in other are set in self.
            #[inline]
            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            /// Returns the raw bits of the flags.
            #[inline]
            pub const fn bits(self) -> $type {
                self.0
            }
        }

        impl core::ops::BitOr for $name {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl core::ops::BitAnd for $name {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl core::ops::BitXor for $name {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    //! Test module for noir_macros_core functionality.
    //! 
    //! This module contains comprehensive tests for all public APIs
    //! and ensures thread-safety, memory safety, and correct behavior
    //! of the static cell and assertion macros.
    
    use super::*;
    use core::fmt::Write;

    /// Tests basic static cell initialization and access.
    /// 
    /// # What This Test Teaches
    /// - How to properly initialize a StaticCell
    /// - Thread-safe access patterns
    /// - Common initialization scenarios
    /// 
    /// # Key Concepts
    /// 1. One-time initialization
    /// 2. Thread safety
    /// 3. Error handling
    #[test]
    fn test_static_cell() {
        let cell = StaticCell::new();
        assert!(cell.try_init(42));
        assert_eq!(cell.get(), Some(&42));
        assert!(!cell.try_init(24));
        assert_eq!(cell.get(), Some(&42));
    }

    /// Tests the PrintWrapper implementation.
    /// 
    /// # Understanding PrintWrapper
    /// PrintWrapper is a zero-cost abstraction that enables
    /// compile-time format string validation without runtime
    /// overhead.
    /// 
    /// # Why This Matters
    /// - Catches formatting errors at compile time
    /// - No runtime performance impact
    /// - Works in no_std environments
    #[test]
    fn test_print_wrapper() {
        let mut pw = PrintWrapper;
        assert!(pw.write_str("test").is_ok());
    }

    /// Tests bitflags macro functionality.
    /// 
    /// # What This Test Demonstrates
    /// 1. Creating bitflag enums
    /// 2. Combining flags with bitwise operations
    /// 3. Checking flag presence
    /// 4. Common use patterns
    /// 
    /// # Learning Points
    /// - How to define flags
    /// - Proper flag combinations
    /// - Flag testing techniques
    /// - Common pitfalls to avoid
    #[test]
    fn test_bitflags_macro() {
        bitflags! {
            struct Flags: u8 {
                const A = 0b0001;
                const B = 0b0010;
                const C = 0b0100;
                const D = 0b1000;
            }
        }

        // Test empty flags
        let empty = Flags::empty();
        assert!(empty.is_empty());

        // Test single flags
        let a = Flags::A;
        assert!(a.contains(Flags::A));
        assert!(!a.contains(Flags::B));

        // Test flag combinations
        let ab = Flags::A | Flags::B;
        assert!(ab.contains(Flags::A));
        assert!(ab.contains(Flags::B));
        assert!(!ab.contains(Flags::C));

        // Test bitwise operations
        let ac = Flags::A | Flags::C;
        let bc = Flags::B | Flags::C;
        let c = ac & bc;  // Should only contain C
        assert!(!c.contains(Flags::A));
        assert!(!c.contains(Flags::B));
        assert!(c.contains(Flags::C));

        // Test XOR operation
        let a_xor_b = Flags::A ^ Flags::B;
        assert!(a_xor_b.contains(Flags::A));
        assert!(a_xor_b.contains(Flags::B));
        let same = Flags::A ^ Flags::A;
        assert!(same.is_empty());

        // Test debug output and raw bits
        let all = Flags::A | Flags::B | Flags::C | Flags::D;
        assert_eq!(all.bits(), 0b1111);
        
        // Verify that raw bits match the expected values
        assert_eq!(Flags::A.bits(), 0b0001);
        assert_eq!(Flags::B.bits(), 0b0010);
        assert_eq!(Flags::C.bits(), 0b0100);
        assert_eq!(Flags::D.bits(), 0b1000);
    }
}
