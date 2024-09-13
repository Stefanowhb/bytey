use crate::{byte_buffer::ByteBuffer, byte_buffer_read::ByteBufferRead, error::Result};
use std::cell::{Cell, RefCell};

impl<T: ByteBufferRead> ByteBufferRead for Cell<T> {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(Cell::new(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(Cell::new(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(Cell::new(buffer.read_be::<T>()?))
    }
}

impl<T: ByteBufferRead> ByteBufferRead for RefCell<T> {
    #[inline]
    fn read_from_bytey_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(RefCell::new(buffer.read::<T>()?))
    }

    #[inline]
    fn read_from_bytey_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(RefCell::new(buffer.read_le::<T>()?))
    }

    #[inline]
    fn read_from_bytey_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
        Ok(RefCell::new(buffer.read_be::<T>()?))
    }
}
