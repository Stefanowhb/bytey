use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};

impl ByteBufferWrite for char {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u32::from(*self).write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u32::from(*self).write_to_bytey_buffer_le(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u32::from(*self).write_to_bytey_buffer_be(buffer)
    }
}
