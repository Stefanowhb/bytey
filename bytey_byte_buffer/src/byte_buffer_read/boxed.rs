use crate::{byte_buffer::ByteBuffer, byte_buffer_read::ByteBufferRead, error::Result};

impl<T: ByteBufferRead> ByteBufferRead for Box<T> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Box<T>> {
        Ok(Box::new(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Box<T>> {
        Ok(Box::new(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Box<T>> {
        Ok(Box::new(buffer.read_be::<T>()?))
    }
}
