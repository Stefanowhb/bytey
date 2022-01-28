use crate::{
    byte_buffer::ByteBuffer,
    error::{ByteBufferError, Result},
    serde_bincode::Decode,
};

impl ByteBuffer {
    pub fn decode<T>(&mut self) -> Result<T>
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

    pub fn decode_le<T>(&mut self) -> Result<T>
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

    pub fn decode_be<T>(&mut self) -> Result<T>
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
