use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};

impl<T: ByteBufferWrite, const N: usize> ByteBufferWrite for [T; N] {
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
