use crate::{byte_buffer::ByteBuffer, byte_buffer_write::ByteBufferWrite, error::Result};

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
