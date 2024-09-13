use crate::byte_buffer_read::ByteBufferRead;
use crate::byte_buffer_write::ByteBufferWrite;
use std::{
    alloc::{self, Layout},
    cmp::max,
    ptr, slice,
};

use crate::error::{ByteBufferError, Result};
use core::ptr::NonNull;
/// A resizeable buffer to store data in.
///
/// Provides a resizeable buffer with an initial capacity of N bytes.
/// All data written to the [`ByteBuffer`] has to implement the [`ByteBufferWrite`] trait or be a slice of type [u8].
///
/// Data read from the [`ByteBuffer`] has to implement the [`ByteBufferRead`] trait.
///
/// # Examples
/// ```
/// use bytey_byte_buffer::byte_buffer::ByteBuffer;
///
/// let mut buffer = ByteBuffer::new().unwrap();
/// let value: u32 = 200;
///
/// // stores the value in the buffer and moves the cursor by 4
/// // due to u32 being 4 bytes in size
/// buffer.write(&value);
///
/// buffer.move_cursor(0);
///
/// // prints 200
/// println!("The stored value is: {}!", buffer.read::<u32>().unwrap());
/// ```
///
#[derive(Debug)]
pub struct ByteBuffer {
    cap: usize,
    length: usize,
    cursor: usize,
    pointer: NonNull<u8>,
}

/// `ByteBuffer` are `Send` Becuase `u8` is `Send` because the data they
/// reference is unaliased.
unsafe impl Send for ByteBuffer {}

/// `ByteBuffer` pointers are `Sync` if `u8` is `Sync` because the data they
/// reference is unaliased.
unsafe impl Sync for ByteBuffer {}

impl ByteBuffer {
    /// The maximum size the [`ByteBuffer`] will allocate.
    ///
    /// The maximum size the [`ByteBuffer`] should be able to allocate is [`isize::MAX`] due to LLVM's GEP Inbounds instruction.
    ///
    /// # Sources
    /// [Rustonomicon](https://doc.rust-lang.org/nomicon/vec/vec-alloc.html)
    ///
    /// [LLVM documentation](https://llvm.org/docs/GetElementPtr.html)
    ///
    pub const MAX_SIZE: usize = isize::MAX as usize;
    // TODO: Add configs to change MIN_SIZE depending on compile target, e.g. the smallest chunk windows 10 64-bit allocates is 24 bytes
    /// The minimum capacity a [`ByteBuffer`] should have in theory.
    ///
    /// Most, if not all, modern operating systems have at least a minimum heap allocation block of 8 bytes.
    /// So it makes little sense to have a [`ByteBuffer`] smaller than 8 bytes.
    pub const MIN_SIZE: usize = 8;

    /// Constructs a new [`ByteBuffer`] of capacity [`MIN_SIZE`](Self::MIN_SIZE)
    ///
    ///
    /// See [`with_capacity`](Self::with_capacity).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// ```
    pub fn new() -> Result<Self> {
        Self::with_capacity(Self::MIN_SIZE)
    }

    /// Constructs a new [`ByteBuffer`] with the given capacity.
    ///
    /// # Errors
    /// - [`ByteBufferError::MinCapacity`] is returned if the given capacity is 0.
    /// - [`ByteBufferError::MaxCapacity`] is returned if the given capacity exceeds [`MAX_SIZE`](Self::MAX_SIZE).
    /// - [`ByteBufferError::AllocationFailure`] is returned if the memory allocation failed due to any reason(see [`alloc::alloc`]).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::with_capacity(256).unwrap();
    /// ```
    pub fn with_capacity(capacity: usize) -> Result<Self> {
        if capacity == 0 {
            return Err(ByteBufferError::MinCapacity);
        } else if capacity > Self::MAX_SIZE {
            return Err(ByteBufferError::MaxCapacity);
        }

        let layout = alloc::Layout::from_size_align(capacity, 1)
            .map_err(|_| ByteBufferError::LayoutFailure { size: capacity })?;

        let new_ptr = unsafe { alloc::alloc(layout) };

        let pointer = match NonNull::new(new_ptr) {
            Some(p) => p,
            None => Err(ByteBufferError::AllocationFailure { size: capacity })?,
        };

        Ok(Self {
            cap: layout.size(),
            length: 0,
            cursor: 0,
            pointer,
        })
    }

    /// Resize the [`ByteBuffer`] to the given capacity.
    ///
    /// # Behaviour
    /// - If the current **length** of the [`ByteBuffer`] exceeds the given capacity, the length will be brought back to equal the given capacity.
    /// - If the current **cursor** position exceeds the length of the buffer, the cursor will be moved back to equal the length of the [`ByteBuffer`].
    ///
    /// To prevent undefined behaviour.
    ///
    /// # Errors
    /// - [`ByteBufferError::MinCapacity`] is returned if the given capacity is 0.
    /// - [`ByteBufferError::MaxCapacity`] is returned if the given capacity exceeds [`MAX_SIZE`](Self::MAX_SIZE).
    /// - [`ByteBufferError::AllocationFailure`] is returned if the memory allocation failed due to any reason(see [`alloc::realloc`]).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    ///
    /// buffer.resize(16);
    /// ```
    pub fn resize(&mut self, capacity: usize) -> Result<&mut Self> {
        if capacity == 0 {
            return Err(ByteBufferError::MinCapacity);
        } else if capacity > Self::MAX_SIZE {
            return Err(ByteBufferError::MaxCapacity);
        }

        let new_layout = alloc::Layout::from_size_align(capacity, 1)
            .map_err(|_| ByteBufferError::LayoutFailure { size: capacity })?;
        let old_layout = alloc::Layout::from_size_align(self.cap, 1)
            .map_err(|_| ByteBufferError::LayoutFailure { size: self.cap })?;

        let new_ptr =
            unsafe { alloc::realloc(self.pointer.as_ptr(), old_layout, new_layout.size()) };

        let pointer = match NonNull::new(new_ptr) {
            Some(p) => p,
            None => Err(ByteBufferError::AllocationFailure { size: capacity })?,
        };

        if self.length >= capacity {
            self.length = capacity;

            if self.cursor >= self.length {
                self.cursor = self.length;
            }
        }

        self.cap = capacity;
        self.pointer = pointer;

        Ok(self)
    }

    /// Expands the capacity of the [`ByteBuffer`] by the given amount.
    ///
    /// # Errors
    /// - [`ByteBufferError::MaxCapacity`] is returned if the given amount results in an overflow on capacity
    ///   or if the result of **capacity + amount** exceeds [`MAX_SIZE`](Self::MAX_SIZE).
    /// - [`ByteBufferError::AllocationFailure`] is returned if the memory allocation failed due to any reason(see [`alloc::realloc`]).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    ///
    /// buffer.expand(4);
    /// ```
    pub fn expand(&mut self, amount: usize) -> Result<&mut Self> {
        self.resize(
            self.cap
                .checked_add(amount)
                .ok_or(ByteBufferError::MaxCapacity)?,
        )
    }

    /// Shrinks the capacity of the [`ByteBuffer`] with a lower bound. The lower bounds is set to 1 if 0.
    /// The capacity will remain at least as large as both the length and the supplied min_capacity.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Errors
    /// - [`ByteBufferError::MinCapacity`] is returned if the given amount results in an underflow on capacity
    ///   or if the result of **capacity - amount** equals 0. This will only occur if Length and min_capacity are both 0.
    /// - [`ByteBufferError::AllocationFailure`] is returned if the memory allocation failed due to any reason(see [`alloc::realloc`]).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    ///
    /// buffer.shrink_to(4);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) -> Result<&mut Self> {
        if self.cap > min_capacity {
            self.resize(max(min_capacity, self.length))?;
        }

        Ok(self)
    }

    /// Shrinks the capacity of the [`ByteBuffer`] as much as possible.
    /// It will drop down as close as possible to the length but the allocator
    /// may still inform the vector that there is space for a few more elements.
    ///
    /// # Errors
    /// - [`ByteBufferError::AllocationFailure`] is returned if the memory allocation failed due to any reason(see [`alloc::realloc`]).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    ///
    /// buffer.shrink_to_fit();
    /// ```
    pub fn shrink_to_fit(&mut self) -> Result<&mut Self> {
        if self.capacity() > self.length {
            self.resize(max(self.length, Self::MIN_SIZE))?;
        }

        Ok(self)
    }

    /// Writes a slice of type [u8] to the [`ByteBuffer`] **without safety checks**.
    ///
    /// # Safety
    ///
    /// This method is unsafe because undefined behaviour can result if the caller does not ensure all of the following:
    /// - The length of the slice doesn't exceed the capacity.
    /// - The cursor position + length of the slice does not exceed the capacity.
    /// - The cursor position is not out of bounds
    ///
    /// # Behaviour
    /// The current cursor position will be increased by the length of the slice.
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let values: [u8; 4] = [0, 1, 2, 3];
    ///
    /// unsafe {
    ///     buffer.write_slice_unchecked(&values);
    /// }
    /// ```
    pub unsafe fn write_slice_unchecked(&mut self, source: &[u8]) -> &mut Self {
        let source_length = source.len();

        ptr::copy_nonoverlapping(
            source.as_ptr(),
            self.pointer.as_ptr().add(self.cursor),
            source_length,
        );
        self.cursor += source.len();

        self
    }

    /// Writes a slice of type [u8] to the [`ByteBuffer`].
    ///
    /// # Behaviour
    /// - If the result of the **current cursor position + length of the slice** exceeds the capacity of the buffer,
    ///   the buffer will resize to the next power of two that fits the result.
    /// - The current cursor position will be increased by the length of the slice.
    ///
    /// # Errors
    /// - [`ByteBufferError::MaxCapacity`] is returned if the buffer has to resize to a capacity larger than [`MAX_SIZE`](Self::MAX_SIZE)
    ///   or if the resulting capacity overflows.
    /// - [`ByteBufferError::AllocationFailure`] is returned if the memory allocation failed due to any reason(see [`alloc::realloc`]).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let values: [u8; 4] = [0, 1, 2, 3];
    ///
    /// buffer.write_slice(&values);
    /// ```
    pub fn write_slice(&mut self, source: &[u8]) -> Result<&mut Self> {
        if self.cursor + source.len() > self.cap {
            let capacity = (self.cursor + source.len())
                .checked_next_power_of_two()
                .ok_or(ByteBufferError::MaxCapacity)?;

            self.resize(capacity)?;
        }

        unsafe {
            self.write_slice_unchecked(source);
        }

        if self.cursor > self.length {
            self.length += self.cursor - self.length
        }

        Ok(self)
    }

    /// Writes the given value to the [`ByteBuffer`].
    ///
    /// The value has to implement the [`ByteBufferWrite`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    /// ```
    pub fn write<T: ByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
        source.write_to_bytey_buffer(self)?;

        Ok(self)
    }

    /// Writes the given value to the [`ByteBuffer`] in **little endian** ordering.
    ///
    /// The value has to implement the [`ByteBufferWrite`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write_le(&value);
    /// ```
    pub fn write_le<T: ByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
        source.write_to_bytey_buffer_le(self)?;

        Ok(self)
    }

    /// Writes the given value to the [`ByteBuffer`] in **big endian** ordering.
    ///
    /// The value has to implement the [`ByteBufferWrite`] trait.
    ///
    /// # Errors & Behaviour
    /// See [`write_slice`](Self::write_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write_be(&value);
    /// ```
    pub fn write_be<T: ByteBufferWrite>(&mut self, source: T) -> Result<&mut Self> {
        source.write_to_bytey_buffer_be(self)?;

        Ok(self)
    }

    /// Reads a slice of type [u8] from the [`ByteBuffer`] of the given size **without safety checks**.
    ///
    /// # Safety
    /// This method is unsafe because undefined behaviour can result if the caller does not ensure all of the following:
    /// - The size does not exceed the capacity of the buffer.
    /// - The result of cursor position + the given size does not exceed the length of the buffer.
    /// - The cursor position is not out of bounds
    ///
    /// # Behaviour
    /// The current cursor position will be increased by the given size.
    ///
    /// # Examples
    ///```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    /// buffer.move_cursor(0);
    ///
    /// unsafe {
    //      println!("{:?}", buffer.read_slice_unchecked(4));
    /// }
    ///```
    pub unsafe fn read_slice_unchecked(&mut self, size: usize) -> &[u8] {
        let ret = slice::from_raw_parts(self.pointer.as_ptr().add(self.cursor), size);
        self.cursor += size;

        ret
    }

    /// Reads a slice of type [u8] from the [`ByteBuffer`] of the given size.
    ///
    /// # Behaviour
    /// The current cursor position will be increased by the given size.
    ///
    /// # Errors
    /// - [`ByteBufferError::ReadOutOfBounds`] is returned if the result of the current cursor position + the given size exceeds the buffer's length
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{:?}", buffer.read_slice(4));
    /// ```
    pub fn read_slice(&mut self, size: usize) -> Result<&[u8]> {
        if self.cursor + size > self.length {
            return Err(ByteBufferError::ReadOutOfBounds {
                length: self.length,
                start: self.cursor,
                end: self.cursor + size,
            });
        }

        Ok(unsafe { self.read_slice_unchecked(size) })
    }

    /// Reads a value of type T that implements the [`ByteBufferRead`] trait from the buffer.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{}", buffer.read::<u32>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let x: u32 = buffer.read().unwrap();
    /// ```
    pub fn read<T: ByteBufferRead>(&mut self) -> Result<T> {
        T::read_from_bytey_buffer(self)
    }

    /// Reads a value of type T that implements the [`ByteBufferRead`] trait from the buffer in **little endian** ordering.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write_le(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{}", buffer.read_le::<u32>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let x: u32 = buffer.read_le().unwrap();
    /// ```
    pub fn read_le<T: ByteBufferRead>(&mut self) -> Result<T> {
        T::read_from_bytey_buffer_le(self)
    }

    /// Reads a value of type T that implements the [`ByteBufferRead`] trait from the buffer in **big endian** ordering.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice).
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write_be(&value);
    /// buffer.move_cursor(0);
    ///
    /// println!("{}", buffer.read_be::<u32>().unwrap());
    /// buffer.move_cursor(0);
    ///
    /// let x: u32 = buffer.read_be().unwrap();
    /// ```
    pub fn read_be<T: ByteBufferRead>(&mut self) -> Result<T> {
        T::read_from_bytey_buffer_be(self)
    }

    /// Moves the current cursor position **without safety checks**.
    ///
    /// # Safety
    /// This method is unsafe because undefined behaviour can result if the caller does not ensure the given location does not exceed the buffer's length.
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// unsafe {
    ///     buffer.move_cursor_unchecked(2);
    /// }
    /// ```
    pub unsafe fn move_cursor_unchecked(&mut self, location: usize) -> &mut Self {
        self.cursor = location;

        self
    }

    /// Moves the current cursor position.
    ///
    /// # Errors
    /// - [`ByteBufferError::CursorOutOfBounds`] if the cursor exceeds the buffers length
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// buffer.move_cursor(2);
    /// ```
    pub fn move_cursor(&mut self, location: usize) -> Result<&mut Self> {
        if location > self.length {
            return Err(ByteBufferError::CursorOutOfBounds {
                length: self.length,
                cursor: location,
            });
        }

        self.cursor = location;

        Ok(self)
    }

    /// Moves the current cursor position to the length of the buffer.
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// buffer.move_cursor();
    /// buffer.move_cursor_to_end();
    /// ```
    pub fn move_cursor_to_end(&mut self) -> &mut Self {
        self.cursor = self.length;
        self
    }

    /// Moves the current cursor position to 0.
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// buffer.move_cursor();
    /// buffer.move_cursor_to_end();
    /// ```
    pub fn move_cursor_to_start(&mut self) -> &mut Self {
        self.cursor = 0;
        self
    }

    /// Resets length without resizing array.
    ///
    /// # Behaviour
    /// buffer's length will be set to length and buffer's cursor will be set to length
    /// if greater than length.
    ///
    /// # Errors
    /// - [`ByteBufferError::LengthOutOfBounds`] is returned if length exceeds the buffer's length
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// let _ = buffer.truncate(0).unwrap();
    /// ```
    pub fn truncate(&mut self, length: usize) -> Result<&mut Self> {
        if length > self.length {
            return Err(ByteBufferError::LengthOutOfBounds {
                current: self.length,
                new: length,
            });
        }

        self.length = length;

        if self.cursor > length {
            self.cursor = length;
        }

        Ok(self)
    }

    /// Returns the length of the [`ByteBuffer`].
    ///
    /// The length of the buffer is the last index written to - 1.
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// println!("{}", buffer.length());
    /// ```
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns the capacity of the [`ByteBuffer`].
    ///
    /// The capacity of the buffer is the size of the heap allocation used to store data.
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    ///
    /// println!("{}", buffer.capacity());
    /// ```
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Returns the current cursor position of the [`ByteBuffer`].
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// println!("{}", buffer.cursor());
    /// ```
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Returns a const pointer to the allocation.
    ///
    /// # Safety
    /// This method is unsafe due to the unsafe nature of pointers itself.
    ///
    /// This method can result in undefined behaviour if the buffer is resized and the underlying heap allocator moves
    /// the pointer
    pub unsafe fn pointer(&self) -> *const u8 {
        self.pointer.as_ptr()
    }

    /// Returns a mutable pointer to the allocation.
    ///
    /// # Safety
    /// This method is unsafe due to the unsafe nature of pointers itself.
    ///
    /// This method can result in undefined behaviour if the buffer is resized and the underlying heap allocator moves
    /// the pointer
    pub unsafe fn mut_pointer(&self) -> *mut u8 {
        self.pointer.as_ptr()
    }

    /// Returns true if the length is 0
    ///
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    /// Returns a new [`ByteBuffer`] with data at the old [`ByteBuffer`]'s Cursor
    /// to the length of the New Buffer. This will move the old buffers cursor.
    ///
    /// # Errors & Behaviour
    /// See [`read_slice`](Self::read_slice), [`with_capacity`](Self::with_capacity) and [`write_slice`](Self::write_slice)
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// buffer.move_cursor(0);
    ///
    /// let mut new_buffer = buffer.read_to_buffer(4).unwrap();
    /// let value = new_buffer.read::<u32>().unwrap();
    /// assert_eq!(value, 12345);
    /// ```
    #[inline]
    pub fn read_to_buffer(&mut self, len: usize) -> Result<Self> {
        let mut buffer = ByteBuffer::with_capacity(len)?;
        let bytes = self.read_slice(len)?;
        buffer.write_slice(bytes)?;
        buffer.cursor = 0;
        Ok(buffer)
    }

    /// Returns a slice of the entire [`ByteBuffer`] from cursor position 0.
    ///
    /// # Behaviour
    /// The current cursor position will be self.length.
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// let slice = buffer.as_slice();
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn as_slice(&mut self) -> &[u8] {
        self.cursor = 0;
        unsafe { self.read_slice_unchecked(self.length()) }
    }

    /// Returns a slice of the entire [`ByteBuffer`] from a given cursor position to a given size.
    ///
    /// # Behaviour
    /// The current cursor position will be the given cursor + given size.
    ///
    /// # Errors
    /// - [`ByteBufferError::ReadOutOfBounds`] is returned if the result of the current cursor position + the given size exceeds the buffer's length
    ///
    /// # Examples
    /// ```
    /// use bytey_byte_buffer::byte_buffer::ByteBuffer;
    ///
    /// let mut buffer = ByteBuffer::new().unwrap();
    /// let value: u32 = 12345;
    ///
    /// buffer.write(&value);
    ///
    /// let slice = buffer.slice_from(0, 4).unwrap();
    /// assert_eq!(slice.len(), 4);
    /// ```
    pub fn slice_from(&mut self, cursor: usize, size: usize) -> Result<&[u8]> {
        if cursor + size > self.length {
            return Err(ByteBufferError::ReadOutOfBounds {
                length: self.length,
                start: cursor,
                end: cursor + size,
            });
        }

        self.cursor = cursor;
        Ok(unsafe { self.read_slice_unchecked(size) })
    }
}

impl Drop for ByteBuffer {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<u8>(self.cap).unwrap();
            alloc::dealloc(self.pointer.as_ptr(), layout);
        }
    }
}

impl Clone for ByteBuffer {
    fn clone(&self) -> Self {
        let layout = alloc::Layout::from_size_align(self.cap, 1).unwrap();
        let pointer = unsafe { alloc::alloc(layout) };
        unsafe {
            ptr::copy(self.pointer.as_ptr(), pointer, self.length);
        }

        Self {
            cap: self.cap,
            length: self.length,
            cursor: self.cursor,
            pointer: NonNull::new(pointer).unwrap(),
        }
    }
}
