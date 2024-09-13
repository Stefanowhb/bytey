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
bytey = "0.4.0"
```

# Usage
```rust
use bytey::ByteBuffer;

fn main() {
    let mut buffer = ByteBuffer::new().unwrap();

    let value1: u16 = 1234;
    let value2: i32 = -2000;
    let value3: usize = usize::MAX;

    // Initially the buffer will have a size of 8 bytes, unless you create the buffer using the with_capacity method
    // The buffer will resize itself to fit all data inside of it
    buffer.write(&value1);
    buffer.write(&value2);
    buffer.write(&value3);

    // When you write a value to the buffer, the cursor will move along
    // So if we want to read the values we just put in, we have to move it back to 0
    buffer.move_cursor(0);

    // Read and print the values stored inside the buffer
    println!("{}", buffer.read::<u16>().unwrap()); // prints "1234"
    println!("{}", buffer.read::<i32>().unwrap()); // prints "-2000"
    println!("{}", buffer.read::<usize>().unwrap()); // prints what the MAX is for usize on the system
}
```
Any value written to the ByteBuffer will have to implement the ``ByteBufferWrite`` trait.
By default, this trait is implemented on all numerical primitives(u8, u16, i8, i16, etc...).

Reading a type from the ByteBuffer requires that type to implement the ``ByteBufferRead`` trait, 
this has also been implemented by default on all numeral primitives.

If you would like to see more default implementations of these traits let me know in an issue on GitHub!

# Macros
Bytey comes with 2 derive macros with the same name as the traits ``ByteBufferWrite`` and ``ByteBufferRead`` 
that you can use on your own structs and enums. 
 
```rust
use bytey::{ByteBuffer, ByteBufferRead, ByteBufferWrite};

fn main() {
    #[derive(ByteBufferRead, ByteBufferWrite, Debug, PartialEq)]
    struct Test {
      a: u8,
      b: u16,
      c: isize,
    }

    let mut buffer = ByteBuffer::new().unwrap();
    let val = Test { a: 1, b: 2, c: 3 };

    buffer.write(&val);
    buffer.move_cursor(0);

    assert_eq!(val, buffer.read::<Test>().unwrap());
}
```
Keep in mind that all the fields inside the struct or enum **must** implement the trait as well, else you will get an error.

# Contributing
Feel free to contribute by sending pull requests. For major changes or if you have an idea that could help improve Bytey, please open an issue!

Please make sure if you do contribute that tests are updated appropriately.

# License
[MIT](https://choosealicense.com/licenses/mit/)