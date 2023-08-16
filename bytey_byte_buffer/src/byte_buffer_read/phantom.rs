use crate::{byte_buffer::ByteBuffer, byte_buffer_read::ByteBufferRead, error::Result};
use std::marker::PhantomData;

impl<T> ByteBufferRead for PhantomData<T> {
    #[inline]
    fn read_from_buffer(_buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(core::marker::PhantomData)
    }

    #[inline]
    fn read_from_buffer_le(_buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(core::marker::PhantomData)
    }

    #[inline]
    fn read_from_buffer_be(_buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(core::marker::PhantomData)
    }
}
