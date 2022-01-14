use crate::byte_buffer::ByteBuffer;
use crate::error::Result;

pub trait ByteBufferRead {
	fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Self> where Self: Sized;
	fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> where Self: Sized;
	fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> where Self: Sized;
}
