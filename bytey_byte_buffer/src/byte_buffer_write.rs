use crate::byte_buffer::ByteBuffer;
use crate::error::Result;
use std::slice;

use cfg_if::cfg_if;

pub trait ByteBufferWrite {
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()>;
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()>;
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()>;
}

impl ByteBufferWrite for u8 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(self as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }
}

impl ByteBufferWrite for &u8 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(*self as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }
}

impl ByteBufferWrite for i8 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(self as *const i8 as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }
}

impl ByteBufferWrite for &i8 {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(*self as *const i8 as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }
}

impl ByteBufferWrite for bool {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let data = *self as u8;
        unsafe {
            buffer.write_slice(slice::from_raw_parts(data as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }
}

impl ByteBufferWrite for &bool {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let data = **self as u8;
        unsafe {
            buffer.write_slice(slice::from_raw_parts(data as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_buffer(buffer)
    }
}

impl<'a, T: ByteBufferWrite + 'a> ByteBufferWrite for Option<T>
where
    &'a T: ByteBufferWrite + 'a,
{
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                buffer.write(1u16)?;
                buffer.write(v)?;
            }
            None => {
                buffer.write(2u16)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                buffer.write_le(1u16)?;
                buffer.write_le(v)?;
            }
            None => {
                buffer.write_le(2u16)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                buffer.write_be(1u16)?;
                buffer.write_be(v)?;
            }
            None => {
                buffer.write_be(2u16)?;
            }
        }

        Ok(())
    }
}

impl<'a, T: ByteBufferWrite> ByteBufferWrite for &Option<T>
where
    &'a T: ByteBufferWrite + 'a,
{
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                buffer.write(1u16)?;
                buffer.write(v)?;
            }
            None => {
                buffer.write(2u16)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                buffer.write_le(1u16)?;
                buffer.write_le(v)?;
            }
            None => {
                buffer.write_le(2u16)?;
            }
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                buffer.write_be(1u16)?;
                buffer.write_be(v)?;
            }
            None => {
                buffer.write_be(2u16)?;
            }
        }

        Ok(())
    }
}

macro_rules! impl_byte_buffer_write_types {
    ($($type:ty),*) => {
        $(
            impl ByteBufferWrite for $type {
                #[inline]
                fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    cfg_if! {
                        if #[cfg(target_endian = "little")] {
                            unsafe {
                                buffer.write_slice(std::slice::from_raw_parts(self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                            }
                        } else {
                            unsafe {
                                let o = self.to_le_bytes();
                                buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                            }
                        }
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    cfg_if! {
                        if #[cfg(target_endian = "big")] {
                            unsafe {
                                buffer.write_slice(std::slice::from_raw_parts(self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                            }
                        } else {
                            unsafe {
                                let o = self.to_be_bytes();
                                buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                            }
                        }
                    }

                    Ok(())
                }
            }

            impl ByteBufferWrite for &$type {
                #[inline]
                fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(*self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    cfg_if! {
                        if #[cfg(target_endian = "little")] {
                            unsafe {
                                buffer.write_slice(std::slice::from_raw_parts(*self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                            }
                        } else {
                            unsafe {
                                let o = self.to_le_bytes();
                                buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                            }
                        }
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    cfg_if! {
                        if #[cfg(target_endian = "big")] {
                            unsafe {
                                buffer.write_slice(std::slice::from_raw_parts(*self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                            }
                        } else {
                            unsafe {
                                let o = self.to_be_bytes();
                                buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                            }
                        }
                    }

                    Ok(())
                }
            }
        )*
    }
}

impl_byte_buffer_write_types!(u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64);
