use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};
use std::borrow::Cow;

impl<T: ByteBufferWrite + Clone> ByteBufferWrite for Cow<'_, T> {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.as_ref().write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.as_ref().write_to_bytey_buffer_le(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.as_ref().write_to_bytey_buffer_be(buffer)
    }
}
