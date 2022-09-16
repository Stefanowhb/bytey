use crate::byte_buffer::ByteBuffer;
use crate::error::Result;

pub trait ByteBufferRead {
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Self>
    where
        Self: Sized;
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Self>
    where
        Self: Sized;
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Self>
    where
        Self: Sized;
}

macro_rules! impl_byte_buffer_read_types {
    ($($type:ty),*) => {
        $(
            impl ByteBufferRead for $type {
                fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<$type> {
                    Ok(<$type>::from_ne_bytes(buffer.read_slice(std::mem::size_of::<$type>())?.try_into().expect("This really shouldn't fail")))
                }

                fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<$type> {
                    Ok(<$type>::from_le_bytes(buffer.read_slice(std::mem::size_of::<$type>())?.try_into().expect("This really shouldn't fail")))
                }

                fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<$type> {
                    Ok(<$type>::from_be_bytes(buffer.read_slice(std::mem::size_of::<$type>())?.try_into().expect("This really shouldn't fail")))
                }
            }
        )*
    }
}

impl ByteBufferRead for bool {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<bool> {
        let data = buffer.read::<u8>()?;

        Ok(data != 0)
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<bool> {
        buffer.read::<bool>()
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<bool> {
        buffer.read::<bool>()
    }
}

impl_byte_buffer_read_types!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
