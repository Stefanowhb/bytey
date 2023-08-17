use crate::{byte_buffer::ByteBuffer, error::Result};

mod arrays;
mod bound;
mod byte;
mod cell;
mod option;
mod phantom;
mod range;
mod result;
mod string;
mod time;
mod tuple;

pub use arrays::*;
pub use bound::*;
pub use byte::*;
pub use cell::*;
pub use option::*;
pub use phantom::*;
pub use range::*;
pub use result::*;
pub use string::*;
pub use time::*;
pub use tuple::*;

pub trait ByteBufferWrite {
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()>;
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()>;
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()>;
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
                    #[cfg(target_endian = "little")]
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    #[cfg(not(target_endian = "little"))]
                    unsafe {
                        let o = self.to_le_bytes();
                        buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    #[cfg(target_endian = "big")]
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    #[cfg(not(target_endian = "big"))]
                    unsafe {
                        let o = self.to_be_bytes();
                        buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
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
                    #[cfg(target_endian = "little")]
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(*self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    #[cfg(not(target_endian = "little"))]
                    unsafe {
                        let o = self.to_le_bytes();
                        buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    #[cfg(target_endian = "big")]
                    unsafe {
                        buffer.write_slice(std::slice::from_raw_parts(*self as *const $type as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    #[cfg(not(target_endian = "big"))]
                    unsafe {
                        let o = self.to_be_bytes();
                        buffer.write_slice(std::slice::from_raw_parts(&o as *const u8, std::mem::size_of::<$type>()))?;
                    }

                    Ok(())
                }
            }
        )*
    }
}

impl_byte_buffer_write_types!(u16, u32, u64, u128, usize, i16, i32, i64, i128, isize, f32, f64);
