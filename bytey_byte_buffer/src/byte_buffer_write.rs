use crate::byte_buffer::ByteBuffer;
use crate::error::Result;

pub trait ByteBufferWrite {
	fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()>;
	fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()>;
	fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()>;
}
