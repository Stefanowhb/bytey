use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};
use std::slice;

impl ByteBufferWrite for u8 {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(self as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }
}

impl ByteBufferWrite for &u8 {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(*self as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }
}

impl ByteBufferWrite for i8 {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(self as *const i8 as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }
}

impl ByteBufferWrite for &i8 {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        unsafe {
            buffer.write_slice(slice::from_raw_parts(*self as *const i8 as *const u8, 1))?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }
}

impl ByteBufferWrite for bool {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let data = if *self { 1u8 } else { 0u8 };
        buffer.write(data)?;

        Ok(())
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }
}

impl ByteBufferWrite for &bool {
    #[inline]
    fn write_to_bytey_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let data = if **self { 1u8 } else { 0u8 };
        buffer.write(data)?;

        Ok(())
    }

    #[inline]
    fn write_to_bytey_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }

    #[inline]
    fn write_to_bytey_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.write_to_bytey_buffer(buffer)
    }
}
