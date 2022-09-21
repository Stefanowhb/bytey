use crate::{
    byte_buffer::ByteBuffer,
    byte_buffer_read::ByteBufferRead,
    error::{ByteBufferError, Result},
};
use std::mem::{self, MaybeUninit};

//The Array read implementation is thanks to bincode. it was designed based on their work.
//If we find any major bugs that could Affect them we should let them know and give them any fixes.
struct Guard<'a, T, const N: usize> {
    array_mut: &'a mut [MaybeUninit<T>; N],
    initialized: usize,
}

impl<T, const N: usize> Drop for Guard<'_, T, N> {
    fn drop(&mut self) {
        debug_assert!(self.initialized <= N);

        // SAFETY: this slice will contain only initialized objects.
        unsafe {
            std::ptr::drop_in_place(
                &mut *(self.array_mut.get_unchecked_mut(..self.initialized)
                    as *mut [MaybeUninit<T>] as *mut [T]),
            );
        }
    }
}

impl<T: ByteBufferRead, const N: usize> ByteBufferRead for [T; N] {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
        if N == 0 {
            return Err(ByteBufferError::OtherError {
                error: "Can not read to an [T;0]. The array must have a size.".to_owned(),
            });
        }

        let size = buffer.read::<usize>()?;

        if size != N {
            return Err(ByteBufferError::OtherError {
                error: format!(
                    "Invalid size in buffer for [T; {}]. Should be [T; {}]?",
                    N, size
                ),
            });
        }

        let mut array = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };
        let mut guard = Guard {
            array_mut: &mut array,
            initialized: 0,
        };

        for _ in 0..N {
            let item = buffer.read::<T>()?;

            // SAFETY: `guard.initialized` starts at 0, is increased by one in the
            // loop and the loop is aborted once it reaches N (which is
            // `array.len()`).
            unsafe {
                guard
                    .array_mut
                    .get_unchecked_mut(guard.initialized)
                    .write(item);
            }
            guard.initialized += 1;
        }

        if guard.initialized == N {
            mem::forget(guard);

            // SAFETY: the condition above asserts that all elements are
            // initialized.
            let out = unsafe { (&array as *const _ as *const [T; N]).read() };
            Ok(out)
        } else {
            Err(ByteBufferError::OtherError {
                error: "Not all of the array was initialized".to_owned(),
            })
        }
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
        if N == 0 {
            return Err(ByteBufferError::OtherError {
                error: "Can not read to an [T;0]. The array must have a size.".to_owned(),
            });
        }

        let size = buffer.read_le::<usize>()?;

        if size != N {
            return Err(ByteBufferError::OtherError {
                error: format!(
                    "Invalid size in buffer for [T; {}]. Should be [T; {}]?",
                    N, size
                ),
            });
        }

        let mut array = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };
        let mut guard = Guard {
            array_mut: &mut array,
            initialized: 0,
        };

        for _ in 0..N {
            let item = buffer.read_le::<T>()?;

            // SAFETY: `guard.initialized` starts at 0, is increased by one in the
            // loop and the loop is aborted once it reaches N (which is
            // `array.len()`).
            unsafe {
                guard
                    .array_mut
                    .get_unchecked_mut(guard.initialized)
                    .write(item);
            }
            guard.initialized += 1;
        }

        if guard.initialized == N {
            mem::forget(guard);

            // SAFETY: the condition above asserts that all elements are
            // initialized.
            let out = unsafe { (&array as *const _ as *const [T; N]).read() };
            Ok(out)
        } else {
            Err(ByteBufferError::OtherError {
                error: "Not all of the array was initialized".to_owned(),
            })
        }
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
        if N == 0 {
            return Err(ByteBufferError::OtherError {
                error: "Can not read to an [T;0]. The array must have a size.".to_owned(),
            });
        }

        let size = buffer.read_be::<usize>()?;

        if size != N {
            return Err(ByteBufferError::OtherError {
                error: format!(
                    "Invalid size in buffer for [T; {}]. Should be [T; {}]?",
                    N, size
                ),
            });
        }

        let mut array = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };
        let mut guard = Guard {
            array_mut: &mut array,
            initialized: 0,
        };

        for _ in 0..N {
            let item = buffer.read_be::<T>()?;

            // SAFETY: `guard.initialized` starts at 0, is increased by one in the
            // loop and the loop is aborted once it reaches N (which is
            // `array.len()`).
            unsafe {
                guard
                    .array_mut
                    .get_unchecked_mut(guard.initialized)
                    .write(item);
            }
            guard.initialized += 1;
        }

        if guard.initialized == N {
            mem::forget(guard);

            // SAFETY: the condition above asserts that all elements are
            // initialized.
            let out = unsafe { (&array as *const _ as *const [T; N]).read() };
            Ok(out)
        } else {
            Err(ByteBufferError::OtherError {
                error: "Not all of the array was initialized".to_owned(),
            })
        }
    }
}

impl<T: ByteBufferRead> ByteBufferRead for Vec<T> {
    #[inline]
    fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
        let size = buffer.read::<usize>()?;

        if size == 0 {
            return Ok(Vec::new());
        }

        let mut vec = Vec::with_capacity(size);

        for _ in 0..size {
            vec.push(buffer.read::<T>()?);
        }

        Ok(vec)
    }

    #[inline]
    fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
        let size = buffer.read_le::<usize>()?;

        if size == 0 {
            return Ok(Vec::new());
        }

        let mut vec = Vec::with_capacity(size);

        for _ in 0..size {
            vec.push(buffer.read_le::<T>()?);
        }

        Ok(vec)
    }

    #[inline]
    fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
        let size = buffer.read_be::<usize>()?;

        if size == 0 {
            return Ok(Vec::new());
        }

        let mut vec = Vec::with_capacity(size);

        for _ in 0..size {
            vec.push(buffer.read_be::<T>()?);
        }

        Ok(vec)
    }
}
