use crate::{byte_buffer::ByteBuffer, byte_buffer_read::ByteBufferRead, error::Result};
use std::borrow::Cow;

impl<T: ByteBufferRead + ToOwned<Owned = T>> ByteBufferRead for Cow<'_, T> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(Cow::Owned(buffer.read_be::<T>()?))
    }
}
