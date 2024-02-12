use crate::{
    byte_buffer::ByteBuffer,
    byte_buffer_read::ByteBufferRead,
    error::{ByteBufferError, Result},
};

impl ByteBufferRead for char {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<char> {
        char::from_u32(buffer.read::<u32>()?).ok_or(ByteBufferError::NotAChar)
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<char> {
        char::from_u32(buffer.read_le::<u32>()?).ok_or(ByteBufferError::NotAChar)
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<char> {
        char::from_u32(buffer.read_be::<u32>()?).ok_or(ByteBufferError::NotAChar)
    }
}
