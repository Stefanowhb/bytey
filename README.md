# Bytey
Bytey provides a convenient and easy to use byte storage.

<p>
    <img src="https://img.shields.io/crates/l/bytey?style=flat-square" />
    <a href="https://crates.io/crates/bytey" alt="Crate">
        <img src="https://img.shields.io/crates/v/bytey?style=flat-square" />
    </a>
    <a href="https://docs.rs/bytey/latest/bytey/" alt="Docs">
        <img src="https://img.shields.io/docsrs/bytey?style=flat-square" />
    </a>
</p>

# Documentation
A link to the documentation can be found [here](https://docs.rs/bytey/latest/bytey/).

# Installation
To start using this crate all you have to do is add it to your ``Cargo.toml``:
```toml
[dependencies]
bytey = "0.1.0"
```

# Usage
```rust
use bytey::ByteBuffer;

fn main() { 
    let mut buffer = ByteBuffer::new().unwrap();
    
    let value1: u16 = 1234;
    let value2: i32 = -2000;
    let value3: usize = usize::MAX;
    
    // The buffer will resize itself to fit all the values
    buffer.write(&value1);
    buffer.write(&value2);
    buffer.write(&value3);
    
    // When you write a value to the buffer, the cursor will move along
    // So if we want to read the values we just put in, we have to move it back to 0
    buffer.move_cursor(0);
    
    // Read and print the values stored inside the buffer
    println!("{}", buffer.read::<u16>().unwrap());
    println!("{}", buffer.read::<i32>().unwrap());
    println!("{}", buffer.read::<usize>().unwrap());
}
```
Any value written to the ByteBuffer will have to implement the ``ByteBufferWrite`` trait.
By default, this trait is implemented on all numerical primitives(u8, u16, i8, i16, etc...).

Reading a type from the ByteBuffer requires that type to implement the ``ByteBufferRead`` trait, 
this has also been implemented by default on all numeral primitives.

If you would like to see more default implementations of these traits let me know in an issue on GitHub!

# Changelog

- **0.2.0**
  - Added feature-gated Bincode support
  - Added Clone trait to ByteBuffer
  - Added truncate method to ByteBuffer

# Contributing
Feel free to contribute by sending pull requests. For major changes or if you have an idea that could help improve Bytey, please open an issue!

Please make sure if you do contribute that tests are updated appropriately.

# License
[MIT](https://choosealicense.com/licenses/mit/)