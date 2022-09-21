use crate::{
    byte_buffer::ByteBuffer,
    byte_buffer_read::ByteBufferRead,
    error::{ByteBufferError, Result},
};
use std::ops::Bound;

impl<T: ByteBufferRead> ByteBufferRead for Bound<T> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
        match buffer.read::<u8>()? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(buffer.read::<T>()?)),
            2 => Ok(Bound::Excluded(buffer.read::<T>()?)),
            _ => Err(ByteBufferError::OtherError {
                error: "Invalid Read to Bound".to_owned(),
            }),
        }
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
        match buffer.read_le::<u8>()? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(buffer.read_le::<T>()?)),
            2 => Ok(Bound::Excluded(buffer.read_le::<T>()?)),
            _ => Err(ByteBufferError::OtherError {
                error: "Invalid Read to Bound".to_owned(),
            }),
        }
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
        match buffer.read_be::<u8>()? {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(buffer.read_be::<T>()?)),
            2 => Ok(Bound::Excluded(buffer.read_be::<T>()?)),
            _ => Err(ByteBufferError::OtherError {
                error: "Invalid Read to Bound".to_owned(),
            }),
        }
    }
}
