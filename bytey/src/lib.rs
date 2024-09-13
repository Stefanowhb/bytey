#![allow(clippy::needless_doctest_main)]
//! # Bytey
//! Bytey provides a convenient and easy to use byte storage.
//! # Installation
//! To start using this crate all you have to do is add it to your **Cargo.toml**:
//! ```toml
//! [dependencies]
//! bytey = "0.4.0"
//! ```
//! # Usage
//!```
//! use bytey::ByteBuffer;
//!
//! fn main() {
//!     let mut buffer = ByteBuffer::new().unwrap();
//!
//!     let value1: u16 = 1234;
//!     let value2: i32 = -2000;
//!     let value3: usize = usize::MAX;
//!
//!     // Initially the buffer will have a size of 8 bytes, unless you create the buffer using the with_capacity method
//!     // The buffer will resize itself to fit all data inside of it
//!     buffer.write(&value1);
//!     buffer.write(&value2);
//!     buffer.write(&value3);
//!
//!     // When you write a value to the buffer, the cursor will move along
//!     // So if we want to read the values we just put in, we have to move it back to 0
//!     buffer.move_cursor(0);
//!
//!     // Read and print the values stored inside the buffer
//!     println!("{}", buffer.read::<u16>().unwrap()); // prints "1234"
//!     println!("{}", buffer.read::<i32>().unwrap()); // prints "-2000"
//!     println!("{}", buffer.read::<usize>().unwrap()); // prints what the MAX is for usize on the system
//! }
//! ```
//! Any value written to the ByteBuffer will have to implement the ``ByteBufferWrite`` trait.
//! By default, this trait is implemented on all numerical primitives(u8, u16, i8, i16, etc...).
//!
//! Reading a type from the ByteBuffer requires that type to implement the ``ByteBufferRead`` trait,
//! this has also been implemented by default on all numeral primitives.
//!
//! If you would like to see more default implementations of these traits let me know in an issue on GitHub!
//! # Contributing
//! Feel free to contribute by sending pull requests. For major changes or if you have an idea that could help improve Bytey, please open an issue!
//!
//! Please make sure if you do contribute that tests are updated appropriately.

#[doc(inline)]
pub use bytey_byte_buffer::byte_buffer::ByteBuffer;

#[doc(inline)]
pub use bytey_byte_buffer::error::{ByteBufferError, Result};

#[doc(inline)]
pub use bytey_byte_buffer::byte_buffer_write::{self, ByteBufferWrite};

#[doc(inline)]
pub use bytey_byte_buffer::byte_buffer_read::{self, ByteBufferRead};

pub use bytey_derive::ByteBufferRead;
pub use bytey_derive::ByteBufferWrite;
