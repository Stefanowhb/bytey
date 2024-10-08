use crate::{
    byte_buffer::ByteBuffer,
    byte_buffer_read::ByteBufferRead,
    error::{ByteBufferError, Result},
};

impl<T: ByteBufferRead> ByteBufferRead for Option<T> {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read::<u8>()? {
            1 => Some(buffer.read::<T>()?),
            2 => None,
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read_le::<u8>()? {
            1 => Some(buffer.read_le::<T>()?),
            2 => None,
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<Option<T>> {
        let data = match buffer.read_be::<u8>()? {
            1 => Some(buffer.read_be::<T>()?),
            2 => None,
            _ => {
                return Err(ByteBufferError::OtherError {
                    error: "Invalid Read to Option".to_owned(),
                })
            }
        };

        Ok(data)
    }
}
