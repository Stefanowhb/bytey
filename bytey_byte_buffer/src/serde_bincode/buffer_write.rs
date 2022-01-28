use crate::{
    byte_buffer::ByteBuffer,
    error::{ByteBufferError, Result},
    serde_bincode::Encode,
};

impl ByteBuffer {
    /// Serializes and Writes the given value to the [`ByteBuffer`].
    ///
    /// The value has to implement the [`bincode::Encode`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    /// See [`encode_to_vec`](bincode::encode_to_vec).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    /// use bytey_byte_buffer::bincode::{self, Encode};
    ///
    /// #[derive(PartialEq, Encode, Debug)]
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
    /// ```
    pub fn encode<T>(&mut self, source: T) -> Result<&mut Self>
    where
        T: Encode,
    {
        let bytes = bincode::encode_to_vec(&source, bincode::config::standard()).map_err(|e| {
            ByteBufferError::OtherError {
                error: e.to_string(),
            }
        })?;

        self.write(bytes.len() as u64)?;
        self.write_slice(&bytes[..])
    }

    /// Serializes and Writes the given value to the [`ByteBuffer`] in **little endian** ordering.
    ///
    /// The value has to implement the [`bincode::Encode`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    /// See [`encode_to_vec`](bincode::encode_to_vec).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    /// use bytey_byte_buffer::bincode::{self, Encode};
    ///
    /// #[derive(PartialEq, Encode, Debug)]
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
    /// ```
    pub fn encode_le<T>(&mut self, source: T) -> Result<&mut Self>
    where
        T: Encode,
    {
        let bytes =
            bincode::encode_to_vec(&source, bincode::config::standard().with_little_endian())
                .map_err(|e| ByteBufferError::OtherError {
                    error: e.to_string(),
                })?;

        self.write_le(bytes.len() as u64)?;
        self.write_slice(&bytes[..])
    }

    /// Serializes and Writes the given value to the [`ByteBuffer`] in **big endian** ordering.
    ///
    /// The value has to implement the [`bincode::Encode`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    /// See [`encode_to_vec`](bincode::encode_to_vec).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    /// use bytey_byte_buffer::bincode::{self, Encode};
    ///
    /// #[derive(PartialEq, Encode, Debug)]
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
    /// ```
    pub fn encode_be<T>(&mut self, source: T) -> Result<&mut Self>
    where
        T: Encode,
    {
        let bytes = bincode::encode_to_vec(&source, bincode::config::standard().with_big_endian())
            .map_err(|e| ByteBufferError::OtherError {
                error: e.to_string(),
            })?;

        self.write_be(bytes.len() as u64)?;
        self.write_slice(&bytes[..])
    }
}
