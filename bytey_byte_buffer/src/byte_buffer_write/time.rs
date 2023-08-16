use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};
use std::time::Duration;

impl ByteBufferWrite for Duration {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.as_secs().write_to_buffer(buffer)?;
        self.subsec_nanos().write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.as_secs().write_to_buffer_le(buffer)?;
        self.subsec_nanos().write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.as_secs().write_to_buffer_be(buffer)?;
        self.subsec_nanos().write_to_buffer_be(buffer)
    }
}
