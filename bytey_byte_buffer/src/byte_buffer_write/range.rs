use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};
use std::ops::{Range, RangeInclusive};

impl<T: ByteBufferWrite> ByteBufferWrite for Range<T> {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.start.write_to_bytey_buffer(buffer)?;
        self.end.write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.start.write_to_bytey_buffer_le(buffer)?;
        self.end.write_to_bytey_buffer_le(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.start.write_to_bytey_buffer_be(buffer)?;
        self.end.write_to_bytey_buffer_be(buffer)
    }
}

impl<T: ByteBufferWrite> ByteBufferWrite for RangeInclusive<T> {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.start().write_to_bytey_buffer(buffer)?;
        self.end().write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.start().write_to_bytey_buffer_le(buffer)?;
        self.end().write_to_bytey_buffer_le(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.start().write_to_bytey_buffer_be(buffer)?;
        self.end().write_to_bytey_buffer_be(buffer)
    }
}
