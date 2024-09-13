use crate::{byte_buffer::ByteBuffer, byte_buffer_read::ByteBufferRead, error::Result};

impl ByteBufferRead for bool {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<bool> {
        Ok(buffer.read::<u8>()? != 0)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<bool> {
        Ok(buffer.read::<u8>()? != 0)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<bool> {
        Ok(buffer.read::<u8>()? != 0)
    }
}
