use crate::{
    byte_buffer::ByteBuffer,
    error::{ByteBufferError, Result},
    serde_bincode::Encode,
};

impl ByteBuffer {
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
