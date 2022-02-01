use bincode::Decode;
use bytey_byte_buffer::{
    byte_buffer::ByteBuffer,
    error::{ByteBufferError, Result},
};

pub trait BincodeDecode {
    fn decode<T>(&mut self) -> Result<T>
    where
        T: Decode;
    fn decode_le<T>(&mut self) -> Result<T>
    where
        T: Decode;
    fn decode_be<T>(&mut self) -> Result<T>
    where
        T: Decode;
}

impl BincodeDecode for ByteBuffer {
    /// Reads a serialized value of type T that implements the [`bincode::Decode`] trait from the buffer.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    /// See [`decode_from_slice`](bincode::decode_from_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    /// use bincode::{self, Decode, Encode};
    /// use bytey_bincode::*;
    ///
    /// #[derive(PartialEq, Decode, Encode, Debug)]
    /// struct TestData {
    ///     x: u32,
    ///     y: u64,
    ///     i: f32,
    /// }
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value = TestData {
    ///     x: 5,
    ///     y: 10,
    ///     i: 5.5,
    /// };
    ///
    /// buffer.encode(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{:?}", buffer.decode::<TestData>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let value: TestData = buffer.decode().unwrap();
    /// ```
    fn decode<T>(&mut self) -> Result<T>
    where
        T: Decode,
    {
        let size = self.read::<u64>()?;
        let source = ByteBuffer::read_slice(self, size as usize)?;

        let (decoded, _): (T, usize) =
            bincode::decode_from_slice(source, bincode::config::standard()).map_err(|e| {
                ByteBufferError::OtherError {
                    error: e.to_string(),
                }
            })?;

        Ok(decoded)
    }

    /// Reads a serialized value of type T that implements the [`bincode::Decode`] trait from the buffer in **little endian** ordering.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    /// See [`decode_from_slice`](bincode::decode_from_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    /// use bincode::{self, Decode, Encode};
    /// use bytey_bincode::*;
    ///
    /// #[derive(PartialEq, Decode, Encode, Debug)]
    /// struct TestData {
    ///     x: u32,
    ///     y: u64,
    ///     i: f32,
    /// }
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value = TestData {
    ///     x: 5,
    ///     y: 10,
    ///     i: 5.5,
    /// };
    ///
    /// buffer.encode_le(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{:?}", buffer.decode_le::<TestData>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let value: TestData = buffer.decode_le().unwrap();
    /// ```
    fn decode_le<T>(&mut self) -> Result<T>
    where
        T: Decode,
    {
        let size = self.read::<u64>()?;
        let source = ByteBuffer::read_slice(self, size as usize)?;

        let (decoded, _): (T, usize) =
            bincode::decode_from_slice(source, bincode::config::standard().with_little_endian())
                .map_err(|e| ByteBufferError::OtherError {
                    error: e.to_string(),
                })?;

        Ok(decoded)
    }

    /// Reads a serialized value of type T that implements the [`bincode::Decode`] trait from the buffer in **big endian** ordering.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    /// See [`decode_from_slice`](bincode::decode_from_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    /// use bincode::{self, Decode, Encode};
    /// use bytey_bincode::*;
    ///
    /// #[derive(PartialEq, Decode, Encode, Debug)]
    /// struct TestData {
    ///     x: u32,
    ///     y: u64,
    ///     i: f32,
    /// }
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value = TestData {
    ///     x: 5,
    ///     y: 10,
    ///     i: 5.5,
    /// };
    ///
    /// buffer.encode_be(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{:?}", buffer.decode_be::<TestData>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let value: TestData = buffer.decode_be().unwrap();
    /// ```
    fn decode_be<T>(&mut self) -> Result<T>
    where
        T: Decode,
    {
        let size = self.read::<u64>()?;
        let source = ByteBuffer::read_slice(self, size as usize)?;

        let (decoded, _): (T, usize) =
            bincode::decode_from_slice(source, bincode::config::standard().with_big_endian())
                .map_err(|e| ByteBufferError::OtherError {
                    error: e.to_string(),
                })?;

        Ok(decoded)
    }
}
