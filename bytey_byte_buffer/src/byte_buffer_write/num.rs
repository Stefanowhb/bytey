use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};

impl ByteBufferWrite for std::num::NonZeroI8 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i8::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i8::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i8::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroU8 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u8::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u8::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u8::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroI16 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i16::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i16::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i16::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroU16 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u16::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u16::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u16::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroI32 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i32::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i32::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i32::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroU32 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u32::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u32::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u32::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroI64 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i64::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i64::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i64::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroU64 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u64::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u64::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u64::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroI128 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i128::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i128::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        i128::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroU128 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u128::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u128::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        u128::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroIsize {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        isize::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        isize::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        isize::from(*self).write_to_buffer_be(buffer)
    }
}

impl ByteBufferWrite for std::num::NonZeroUsize {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        usize::from(*self).write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        usize::from(*self).write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        usize::from(*self).write_to_buffer_be(buffer)
    }
}

impl<T: ByteBufferWrite + Copy> ByteBufferWrite for std::num::Wrapping<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.0.write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.0.write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.0.write_to_buffer_be(buffer)
    }
}

impl<T: ByteBufferWrite + Copy> ByteBufferWrite for std::num::Saturating<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.0.write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.0.write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.0.write_to_buffer_be(buffer)
    }
}
