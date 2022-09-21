use crate::{
    byte_buffer::ByteBuffer,
    error::{ByteBufferError, Result},
};

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
        Ok(buffer.read::<u8>()? != 0)
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<bool> {
        Ok(buffer.read::<u8>()? != 0)
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<bool> {
        Ok(buffer.read::<u8>()? != 0)
    }
}

impl_byte_buffer_read_types!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl<T: ByteBufferRead> ByteBufferRead for Option<T> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read::<u16>()? {
            1 => Some(buffer.read::<T>()?),
            2 => None,
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read_le::<u16>()? {
            1 => Some(buffer.read_le::<T>()?),
            2 => None,
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read_be::<u16>()? {
            1 => Some(buffer.read_be::<T>()?),
            2 => None,
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }
}
