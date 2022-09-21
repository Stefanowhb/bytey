use crate::{
    byte_buffer::ByteBuffer,
    byte_buffer_read::ByteBufferRead,
    error::{ByteBufferError, Result},
};

macro_rules! array_impls {
    ($($len:expr => ($($n:tt)+))+) => {
        $(
            impl<T: ByteBufferRead> ByteBufferRead for [T; $len]
            {
                #[inline]
                fn read_from_buffer(buffer: &mut ByteBuffer) -> Result<Self> {
                    let size = buffer.read::<usize>()?;

                    if size != $len {
                        return Err(ByteBufferError::OtherError {
                            error: format!("Invalid size in buffer for [T; {}]. Should be [T; {}]?", $len, size),
                        })
                    }

                    Ok([$(
                        match buffer.read::<T>() {
                            Ok(v) => v,
                            Err(e) => return Err(ByteBufferError::OtherError {
                                error: format!("{} occured at arr[{}] ?", e, $n),
                            })
                        }
                    ),+])
                }

                #[inline]
                fn read_from_buffer_le(buffer: &mut ByteBuffer) -> Result<Self> {
                    let size = buffer.read_le::<usize>()?;

                    if size != $len {
                        return Err(ByteBufferError::OtherError {
                            error: format!("Invalid size in buffer for [T; {}]. Should be [T; {}]?", $len, size),
                        })
                    }

                    Ok([$(
                        match buffer.read_le::<T>() {
                            Ok(v) => v,
                            Err(e) => return Err(ByteBufferError::OtherError {
                                error: format!("{} occured at arr[{}] ?", e, $n),
                            })
                        }
                    ),+])
                }

                #[inline]
                fn read_from_buffer_be(buffer: &mut ByteBuffer) -> Result<Self> {
                    let size = buffer.read_be::<usize>()?;

                    if size != $len {
                        return Err(ByteBufferError::OtherError {
                            error: format!("Invalid size in buffer for [T; {}]. Should be [T; {}]?", $len, size),
                        })
                    }

                    Ok([$(
                        match buffer.read_be::<T>() {
                            Ok(v) => v,
                            Err(e) => return Err(ByteBufferError::OtherError {
                                error: format!("{} occured at arr[{}] ?", e, $n),
                            })
                        }
                    ),+])
                }
            }
        )+
    }
}

array_impls! {
    1 => (0)
    2 => (0 1)
    3 => (0 1 2)
    4 => (0 1 2 3)
    5 => (0 1 2 3 4)
    6 => (0 1 2 3 4 5)
    7 => (0 1 2 3 4 5 6)
    8 => (0 1 2 3 4 5 6 7)
    9 => (0 1 2 3 4 5 6 7 8)
    10 => (0 1 2 3 4 5 6 7 8 9)
    11 => (0 1 2 3 4 5 6 7 8 9 10)
    12 => (0 1 2 3 4 5 6 7 8 9 10 11)
    13 => (0 1 2 3 4 5 6 7 8 9 10 11 12)
    14 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13)
    15 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14)
    16 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15)
    17 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16)
    18 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17)
    19 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18)
    20 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19)
    21 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20)
    22 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21)
    23 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22)
    24 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23)
    25 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24)
    26 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25)
    27 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26)
    28 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27)
    29 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28)
    30 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29)
    31 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30)
    32 => (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31)
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
