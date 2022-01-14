use std::{alloc, ptr, slice};
use crate::byte_buffer_read::ByteBufferRead;
use crate::byte_buffer_write::ByteBufferWrite;

use crate::error::{Result, ByteBufferError};

pub struct ByteBuffer {
	layout: alloc::Layout,
	length: usize,
	cursor: usize,
	pointer: *mut u8
}

impl ByteBuffer {
	/// The maximum size the buffer will allocate
	///
	/// The maximum size the buffer should be able to allocate is [`isize::MAX`] due to LLVM's GEP Inbounds instruction.
	///
	/// # Sources
	///	[Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec-alloc.html)
	///
	/// [LLVM documentation](https://llvm.org/docs/GetElementPtr.html)
	///
	const MAX_SIZE: usize = isize::MAX as usize;
	// TODO: Add configs to change MIN_SIZE depending on compile target, e.g. the smallest chunk windows 10 64-bit allocates is 24 bytes
	const MIN_SIZE: usize = 8;

	pub fn new() -> Result<Self> {
		Self::with_capacity(Self::MIN_SIZE)
	}

	pub fn with_capacity(capacity: usize) -> Result<Self> {
		if capacity == 0 {
			return Err(ByteBufferError::MinCapacity);
		} else if capacity > Self::MAX_SIZE {
			return Err(ByteBufferError::MaxCapacity);
		}

		let layout = unsafe {
			alloc::Layout::from_size_align_unchecked(capacity, 1)
		};

		let pointer = unsafe {
			alloc::alloc(layout)
		};

		if pointer.is_null() {
			return Err(ByteBufferError::AllocationFailure {
				size: capacity
			});
		}

		Ok(Self {
			layout,
			length: 0,
			cursor: 0,
			pointer
		})
	}

	pub fn resize(&mut self, capacity: usize) -> Result<&mut Self> {
		if capacity == 0 {
			return Err(ByteBufferError::MinCapacity);
		} else if capacity > Self::MAX_SIZE {
			return Err(ByteBufferError::MaxCapacity);
		}

		let layout = unsafe {
			alloc::Layout::from_size_align_unchecked(capacity, 1)
		};
		let pointer = unsafe {
			alloc::realloc(self.pointer, layout, capacity)
		};

		if pointer.is_null() {
			return Err(ByteBufferError::AllocationFailure {
				size: capacity
			});
		}

		if self.length >= capacity {
			self.length = capacity;

			if self.cursor >= self.length {
				self.cursor = self.length;
			}
		}

		self.layout = layout;
		self.pointer = pointer;

		Ok(self)
	}

	pub fn expand(&mut self, amount: usize) -> Result<&mut Self> {
		self.resize(self.layout.size().checked_add(amount).ok_or(ByteBufferError::MaxCapacity)?)
	}

	pub fn shrink(&mut self, amount: usize) -> Result<&mut Self> {
		self.resize(self.layout.size().checked_sub(amount).ok_or(ByteBufferError::MaxCapacity)?)
	}

	pub unsafe fn write_slice_unchecked(&mut self, source: &[u8]) -> &mut Self {
		let source_length = source.len();

		ptr::copy_nonoverlapping(source.as_ptr(), self.pointer.add(self.cursor), source_length);
		self.cursor += source.len();

		self
	}

	pub fn write_slice(&mut self, source: &[u8]) -> Result<&mut Self> {
		if self.cursor + source.len() > self.layout.size() {
			let capacity = (self.cursor + source.len()).checked_next_power_of_two().ok_or(ByteBufferError::MaxCapacity)?;

			let layout = unsafe {
				alloc::Layout::from_size_align_unchecked(capacity, 1)
			};
			let pointer = unsafe {
				alloc::realloc(self.pointer, layout, capacity)
			};

			if pointer.is_null() {
				return Err(ByteBufferError::AllocationFailure {
					size: layout.size()
				});
			}

			self.layout = layout;
			self.pointer = pointer;
		}

		unsafe {
			self.write_slice_unchecked(source);
		}

		if self.cursor > self.length {
			self.length += self.cursor - self.length
		}

		Ok(self)
	}

	pub fn write<T: ByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
		source.write_to_buffer(self)?;

		Ok(self)
	}

	pub fn write_le<T: ByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
		source.write_to_buffer_le(self)?;

		Ok(self)
	}

	pub fn write_be<T: ByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
		source.write_to_buffer_be(self)?;

		Ok(self)
	}

	pub unsafe fn read_slice_unchecked(&mut self, size: usize) -> &[u8] {
		let ret = slice::from_raw_parts(self.pointer.add(self.cursor), size);
		self.cursor += size;

		ret
	}

	pub fn read_slice(&mut self, size: usize) -> Result<&[u8]> {
		if self.cursor + size > self.length {
			return Err(ByteBufferError::ReadOutOfBounds {
				length: self.length,
				start: self.cursor,
				end: self.cursor + size
			});
		}

		Ok(unsafe {
			self.read_slice_unchecked(size)
		})
	}

	pub fn read<T: ByteBufferRead>(&mut self) -> Result<T> {
		T::read_from_buffer(self)
	}

	pub fn read_le<T: ByteBufferRead>(&mut self) -> Result<T> {
		T::read_from_buffer_le(self)
	}

	pub fn read_be<T: ByteBufferRead>(&mut self) -> Result<T> {
		T::read_from_buffer_be(self)
	}

	pub unsafe fn move_cursor_unchecked(&mut self, location: usize) -> &mut Self {
		self.cursor = location;

		self
	}

	pub fn move_cursor(&mut self, location: usize) -> Result<&mut Self> {
		if location > self.length {
			return Err(ByteBufferError::CursorOutOfBounds {
				length: self.length,
				cursor: location
			})
		}

		self.cursor = location;

		Ok(self)
	}

	pub fn length(&self) -> usize {
		self.length
	}

	pub fn capacity(&self) -> usize {
		self.layout.size()
	}

	pub fn cursor(&self) -> usize {
		self.cursor
	}

	pub fn layout(&self) -> alloc::Layout {
		self.layout
	}

	pub unsafe fn pointer(&self) -> *const u8 {
		self.pointer
	}

	pub unsafe fn mut_pointer(&self) -> *mut u8 {
		self.pointer
	}
}

impl Drop for ByteBuffer {
	fn drop(&mut self) {
		unsafe {
			alloc::dealloc(self.pointer, self.layout);
		}
	}
}