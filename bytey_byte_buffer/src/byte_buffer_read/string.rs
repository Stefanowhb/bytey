use crate::{byte_buffer::ByteBuffer, byte_buffer_read::ByteBufferRead, error::Result};

impl ByteBufferRead for String {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
        let len = buffer.read::<usize>()?;

        if len == 0 {
            Ok(String::new())
        } else {
            Ok(std::str::from_utf8(buffer.read_slice(len)?)?.to_owned())
        }
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
        let len = buffer.read_le::<usize>()?;

        if len == 0 {
            Ok(String::new())
        } else {
            Ok(std::str::from_utf8(buffer.read_slice(len)?)?.to_owned())
        }
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
        let len = buffer.read_be::<usize>()?;

        if len == 0 {
            Ok(String::new())
        } else {
            Ok(std::str::from_utf8(buffer.read_slice(len)?)?.to_owned())
        }
    }
}
