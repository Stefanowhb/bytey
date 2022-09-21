use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};

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
