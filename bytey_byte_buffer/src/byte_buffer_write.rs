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

impl<T: ByteBufferWrite> ByteBufferWrite for Option<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u16.write_to_buffer(buffer)?;
                v.write_to_buffer(buffer)
            }
            None => 2u16.write_to_buffer(buffer),
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u16.write_to_buffer_le(buffer)?;
                v.write_to_buffer_le(buffer)
            }
            None => 2u16.write_to_buffer_le(buffer),
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Some(v) => {
                1u16.write_to_buffer_be(buffer)?;
                v.write_to_buffer_be(buffer)
            }
            None => 2u16.write_to_buffer_be(buffer),
        }
    }
}

impl<T: ByteBufferWrite> ByteBufferWrite for &Option<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u16.write_to_buffer(buffer)?;
                v.write_to_buffer(buffer)
            }
            None => 2u16.write_to_buffer(buffer),
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u16.write_to_buffer_le(buffer)?;
                v.write_to_buffer_le(buffer)
            }
            None => 2u16.write_to_buffer_le(buffer),
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Some(v) => {
                1u16.write_to_buffer_be(buffer)?;
                v.write_to_buffer_be(buffer)
            }
            None => 2u16.write_to_buffer_be(buffer),
        }
    }
}

impl<T: ByteBufferWrite, E: ByteBufferWrite> ByteBufferWrite for std::result::Result<T, E> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u16.write_to_buffer(buffer)?;
                v.write_to_buffer(buffer)
            }
            Err(e) => {
                2u16.write_to_buffer(buffer)?;
                e.write_to_buffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u16.write_to_buffer_le(buffer)?;
                v.write_to_buffer_le(buffer)
            }
            Err(e) => {
                2u16.write_to_buffer_le(buffer)?;
                e.write_to_buffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u16.write_to_buffer_be(buffer)?;
                v.write_to_buffer_be(buffer)
            }
            Err(e) => {
                2u16.write_to_buffer_be(buffer)?;
                e.write_to_buffer_be(buffer)
            }
        }
    }
}

impl<T: ByteBufferWrite, E: ByteBufferWrite> ByteBufferWrite for &std::result::Result<T, E> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Ok(v) => {
                1u16.write_to_buffer(buffer)?;
                v.write_to_buffer(buffer)
            }
            Err(e) => {
                2u16.write_to_buffer(buffer)?;
                e.write_to_buffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Ok(v) => {
                1u16.write_to_buffer_le(buffer)?;
                v.write_to_buffer_le(buffer)
            }
            Err(e) => {
                2u16.write_to_buffer_le(buffer)?;
                e.write_to_buffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Ok(v) => {
                1u16.write_to_buffer_be(buffer)?;
                v.write_to_buffer_be(buffer)
            }
            Err(e) => {
                2u16.write_to_buffer_be(buffer)?;
                e.write_to_buffer_be(buffer)
            }
        }
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

macro_rules! array_impls {
    ($($len:tt)+) => {
        $(
            impl<T: ByteBufferWrite> ByteBufferWrite for [T; $len]
            {
                #[inline]
                fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    self.len().write_to_buffer(buffer)?;

                    for e in self {
                        e.write_to_buffer(buffer)?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    self.len().write_to_buffer_le(buffer)?;

                    for e in self {
                        e.write_to_buffer_le(buffer)?;
                    }

                    Ok(())
                }

                #[inline]
                fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
                    self.len().write_to_buffer_be(buffer)?;

                    for e in self {
                        e.write_to_buffer_be(buffer)?;
                    }

                    Ok(())
                }
            }
        )+
    }
}

array_impls! {
    1 2 3 4 5 6 7 8 9 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}

impl<T: ByteBufferWrite> ByteBufferWrite for [T] {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer(buffer)?;

        for e in self {
            e.write_to_buffer(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer_le(buffer)?;

        for e in self {
            e.write_to_buffer_le(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer_be(buffer)?;

        for e in self {
            e.write_to_buffer_be(buffer)?;
        }

        Ok(())
    }
}

impl<T: ByteBufferWrite> ByteBufferWrite for Vec<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer(buffer)?;

        for e in self {
            e.write_to_buffer(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer_le(buffer)?;

        for e in self {
            e.write_to_buffer_le(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer_be(buffer)?;

        for e in self {
            e.write_to_buffer_be(buffer)?;
        }

        Ok(())
    }
}

impl<T: ByteBufferWrite> ByteBufferWrite for &Vec<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer(buffer)?;

        for e in *self {
            e.write_to_buffer(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer_le(buffer)?;

        for e in *self {
            e.write_to_buffer_le(buffer)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.len().write_to_buffer_be(buffer)?;

        for e in *self {
            e.write_to_buffer_be(buffer)?;
        }

        Ok(())
    }
}
