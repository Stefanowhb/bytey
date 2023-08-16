use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};

impl<T: ByteBufferWrite, E: ByteBufferWrite> ByteBufferWrite for std::result::Result<T, E> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u8.write_to_buffer(buffer)?;
                v.write_to_buffer(buffer)
            }
            Err(e) => {
                2u8.write_to_buffer(buffer)?;
                e.write_to_buffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u8.write_to_buffer_le(buffer)?;
                v.write_to_buffer_le(buffer)
            }
            Err(e) => {
                2u8.write_to_buffer_le(buffer)?;
                e.write_to_buffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(v) => {
                1u8.write_to_buffer_be(buffer)?;
                v.write_to_buffer_be(buffer)
            }
            Err(e) => {
                2u8.write_to_buffer_be(buffer)?;
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
                1u8.write_to_buffer(buffer)?;
                v.write_to_buffer(buffer)
            }
            Err(e) => {
                2u8.write_to_buffer(buffer)?;
                e.write_to_buffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Ok(v) => {
                1u8.write_to_buffer_le(buffer)?;
                v.write_to_buffer_le(buffer)
            }
            Err(e) => {
                2u8.write_to_buffer_le(buffer)?;
                e.write_to_buffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Ok(v) => {
                1u8.write_to_buffer_be(buffer)?;
                v.write_to_buffer_be(buffer)
            }
            Err(e) => {
                2u8.write_to_buffer_be(buffer)?;
                e.write_to_buffer_be(buffer)
            }
        }
    }
}

impl<E: ByteBufferWrite> ByteBufferWrite for std::result::Result<(), E> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(()) => 1u8.write_to_buffer(buffer),
            Err(e) => {
                2u8.write_to_buffer(buffer)?;
                e.write_to_buffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(()) => 1u8.write_to_buffer_le(buffer),
            Err(e) => {
                2u8.write_to_buffer_le(buffer)?;
                e.write_to_buffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match self {
            Ok(()) => 1u8.write_to_buffer_be(buffer),
            Err(e) => {
                2u8.write_to_buffer_be(buffer)?;
                e.write_to_buffer_be(buffer)
            }
        }
    }
}

impl<E: ByteBufferWrite> ByteBufferWrite for &std::result::Result<(), E> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Ok(()) => 1u8.write_to_buffer(buffer),
            Err(e) => {
                2u8.write_to_buffer(buffer)?;
                e.write_to_buffer(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Ok(()) => 1u8.write_to_buffer_le(buffer),
            Err(e) => {
                2u8.write_to_buffer_le(buffer)?;
                e.write_to_buffer_le(buffer)
            }
        }
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        match *self {
            Ok(()) => 1u8.write_to_buffer_be(buffer),
            Err(e) => {
                2u8.write_to_buffer_be(buffer)?;
                e.write_to_buffer_be(buffer)
            }
        }
    }
}
