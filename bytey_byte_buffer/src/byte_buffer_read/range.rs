use crate::{byte_buffer::ByteBuffer, byte_buffer_read::ByteBufferRead, error::Result};
use std::ops::{Range, RangeInclusive};

impl<T: ByteBufferRead> ByteBufferRead for Range<T> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(buffer.read::<T>()?..buffer.read::<T>()?)
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(buffer.read::<T>()?..buffer.read::<T>()?)
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(buffer.read::<T>()?..buffer.read::<T>()?)
    }
}

impl<T: ByteBufferRead> ByteBufferRead for RangeInclusive<T> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<bool> {
        Ok(RangeInclusive::new(
            buffer.read::<T>()?,
            buffer.read::<T>()?,
        ))
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<bool> {
        Ok(RangeInclusive::new(
            buffer.read::<T>()?,
            buffer.read::<T>()?,
        ))
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<bool> {
        Ok(RangeInclusive::new(
            buffer.read::<T>()?,
            buffer.read::<T>()?,
        ))
    }
}
