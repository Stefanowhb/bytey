use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};
use std::marker::PhantomData;

impl<T> ByteBufferWrite for PhantomData<T> {
    #[inline]
    fn write_to_buffer(&self, _buffer: &mut ByteBuffer) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, _buffer: &mut ByteBuffer) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, _buffer: &mut ByteBuffer) -> Result<()> {
        Ok(())
    }
}
