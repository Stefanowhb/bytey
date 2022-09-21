use crate::{
    byte_buffer::ByteBuffer,
    byte_buffer_read::ByteBufferRead,
    error::{ByteBufferError, Result},
};

impl<T: ByteBufferRead, E: ByteBufferRead> ByteBufferRead for std::result::Result<T, E> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<std::result::Result<T, E>> {
        Ok(match buffer.read::<u16>()? {
            1 => Ok(buffer.read::<T>()?),
            2 => Err(buffer.read::<E>()?),
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<std::result::Result<T, E>> {
        Ok(match buffer.read_le::<u16>()? {
            1 => Ok(buffer.read_le::<T>()?),
            2 => Err(buffer.read_le::<E>()?),
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<std::result::Result<T, E>> {
        Ok(match buffer.read_be::<u16>()? {
            1 => Ok(buffer.read_be::<T>()?),
            2 => Err(buffer.read_be::<E>()?),
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }
}

impl<E: ByteBufferRead> ByteBufferRead for std::result::Result<(), E> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<std::result::Result<(), E>> {
        Ok(match buffer.read::<u16>()? {
            1 => Ok(()),
            2 => Err(buffer.read::<E>()?),
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<std::result::Result<(), E>> {
        Ok(match buffer.read_le::<u16>()? {
            1 => Ok(()),
            2 => Err(buffer.read_le::<E>()?),
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<std::result::Result<(), E>> {
        Ok(match buffer.read_be::<u16>()? {
            1 => Ok(()),
            2 => Err(buffer.read_be::<E>()?),
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Result".to_owned(),
                })
            }
        })
    }
}
