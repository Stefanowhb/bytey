use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};
use std::ops::Bound;

impl<T: ByteBufferWrite> ByteBufferWrite for Bound<T> {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Self::Unbounded => 0u8.write_to_bytey_buffer(buffer),
            Self::Included(val) => {
                1u8.write_to_bytey_buffer(buffer)?;
                val.write_to_bytey_buffer(buffer)
            }
            Self::Excluded(val) => {
                2u8.write_to_bytey_buffer(buffer)?;
                val.write_to_bytey_buffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Self::Unbounded => 0u8.write_to_bytey_buffer_le(buffer),
            Self::Included(val) => {
                1u8.write_to_bytey_buffer_le(buffer)?;
                val.write_to_bytey_buffer_le(buffer)
            }
            Self::Excluded(val) => {
                2u8.write_to_bytey_buffer_le(buffer)?;
                val.write_to_bytey_buffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Self::Unbounded => 0u8.write_to_bytey_buffer_be(buffer),
            Self::Included(val) => {
                1u8.write_to_bytey_buffer_be(buffer)?;
                val.write_to_bytey_buffer_be(buffer)
            }
            Self::Excluded(val) => {
                2u8.write_to_bytey_buffer_be(buffer)?;
                val.write_to_bytey_buffer_be(buffer)
            }
        }
    }
}
