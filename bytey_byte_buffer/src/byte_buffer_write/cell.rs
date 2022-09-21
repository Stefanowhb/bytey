use crate::{
    byte_buffer::ByteBuffer,
    byte_buffer_write::ByteBufferWrite,
    error::{ByteBufferError, Result},
};
use std::cell::{Cell, RefCell};

impl<T: ByteBufferWrite + Copy> ByteBufferWrite for Cell<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.get().write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.get().write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.get().write_to_buffer_be(buffer)
    }
}

impl<T: ByteBufferWrite + ?Sized> ByteBufferWrite for RefCell<T> {
    #[inline]
    fn write_to_buffer(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.try_borrow()
            .map_err(|e| ByteBufferError::RefCellAlreadyBorrowed {
                error: e.to_string(),
                type_name: core::any::type_name::<RefCell<T>>(),
            })?
            .write_to_buffer(buffer)
    }

    #[inline]
    fn write_to_buffer_le(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.try_borrow()
            .map_err(|e| ByteBufferError::RefCellAlreadyBorrowed {
                error: e.to_string(),
                type_name: core::any::type_name::<RefCell<T>>(),
            })?
            .write_to_buffer_le(buffer)
    }

    #[inline]
    fn write_to_buffer_be(&self, buffer: &mut ByteBuffer) -> Result<()> {
        self.try_borrow()
            .map_err(|e| ByteBufferError::RefCellAlreadyBorrowed {
                error: e.to_string(),
                type_name: core::any::type_name::<RefCell<T>>(),
            })?
            .write_to_buffer_be(buffer)
    }
}
