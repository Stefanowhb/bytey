use crate::{
    byte_buffer::ByteBuffer,
    byte_buffer_read::ByteBufferRead,
    error::{ByteBufferError, Result},
};
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Saturating, Wrapping,
};

impl ByteBufferRead for NonZeroI8 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroI8> {
        NonZeroI8::new(buffer.read::<i8>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroI8> {
        NonZeroI8::new(buffer.read::<i8>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroI8> {
        NonZeroI8::new(buffer.read::<i8>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroU8 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroU8> {
        NonZeroU8::new(buffer.read::<u8>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroU8> {
        NonZeroU8::new(buffer.read::<u8>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroU8> {
        NonZeroU8::new(buffer.read::<u8>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroI16 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroI16> {
        NonZeroI16::new(buffer.read::<i16>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroI16> {
        NonZeroI16::new(buffer.read_le::<i16>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroI16> {
        NonZeroI16::new(buffer.read_be::<i16>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroU16 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroU16> {
        NonZeroU16::new(buffer.read::<u16>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroU16> {
        NonZeroU16::new(buffer.read_le::<u16>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroU16> {
        NonZeroU16::new(buffer.read_be::<u16>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroI32 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroI32> {
        NonZeroI32::new(buffer.read::<i32>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroI32> {
        NonZeroI32::new(buffer.read_le::<i32>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroI32> {
        NonZeroI32::new(buffer.read_be::<i32>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroU32 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroU32> {
        NonZeroU32::new(buffer.read::<u32>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroU32> {
        NonZeroU32::new(buffer.read_le::<u32>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroU32> {
        NonZeroU32::new(buffer.read_be::<u32>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroI64 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroI64> {
        NonZeroI64::new(buffer.read::<i64>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroI64> {
        NonZeroI64::new(buffer.read_le::<i64>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroI64> {
        NonZeroI64::new(buffer.read_be::<i64>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroU64 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroU64> {
        NonZeroU64::new(buffer.read::<u64>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroU64> {
        NonZeroU64::new(buffer.read_le::<u64>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroU64> {
        NonZeroU64::new(buffer.read_be::<u64>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroI128 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroI128> {
        NonZeroI128::new(buffer.read::<i128>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroI128> {
        NonZeroI128::new(buffer.read_le::<i128>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroI128> {
        NonZeroI128::new(buffer.read_be::<i128>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroU128 {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroU128> {
        NonZeroU128::new(buffer.read::<u128>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroU128> {
        NonZeroU128::new(buffer.read_le::<u128>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroU128> {
        NonZeroU128::new(buffer.read_be::<u128>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroIsize {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroIsize> {
        NonZeroIsize::new(buffer.read::<isize>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroIsize> {
        NonZeroIsize::new(buffer.read_le::<isize>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroIsize> {
        NonZeroIsize::new(buffer.read_be::<isize>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl ByteBufferRead for NonZeroUsize {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<NonZeroUsize> {
        NonZeroUsize::new(buffer.read::<usize>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<NonZeroUsize> {
        NonZeroUsize::new(buffer.read_le::<usize>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<NonZeroUsize> {
        NonZeroUsize::new(buffer.read_be::<usize>()?).ok_or(ByteBufferError::NonZeroIsZero)
    }
}

impl<T: ByteBufferRead> ByteBufferRead for Wrapping<T> {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<Wrapping<T>> {
        Ok(Wrapping(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<Wrapping<T>> {
        Ok(Wrapping(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<Wrapping<T>> {
        Ok(Wrapping(buffer.read_be::<T>()?))
    }
}

impl<T: ByteBufferRead> ByteBufferRead for Saturating<T> {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<Saturating<T>> {
        Ok(Saturating(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<Saturating<T>> {
        Ok(Saturating(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<Saturating<T>> {
        Ok(Saturating(buffer.read_be::<T>()?))
    }
}
