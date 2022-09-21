use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};

impl ByteBufferWrite for str {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer_le(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer_be(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }
}

impl<'a> ByteBufferWrite for &'a str {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer_le(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer_be(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }
}

impl ByteBufferWrite for String {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer_le(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer_be(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }
}

impl ByteBufferWrite for &String {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer_le(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        let bytestr = self.as_bytes();
        let len = bytestr.len();
        len.write_to_buffer_be(buffer)?;

        if len > 0 {
            buffer.write_slice(bytestr)?;
        }

        Ok(())
    }
}
