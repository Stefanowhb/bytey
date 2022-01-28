use bytey_byte_buffer::byte_buffer::ByteBuffer;

macro_rules! test_default_impl_ne {
    ($type:ty) => {
        let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<$type>()).unwrap();
        let value: $type = <$type>::MAX / 2 as $type;
        let value_bytes = value.to_ne_bytes();

        let _ = buffer.write(&value);
        let _ = buffer.move_cursor(0);

        assert_eq!(buffer.read::<$type>().unwrap(), value);

        let _ = buffer.move_cursor(0);

        assert_eq!(
            buffer.read_slice(std::mem::size_of::<$type>()).unwrap(),
            value_bytes
        );
    };
}

macro_rules! test_default_impl_le {
    ($type:ty) => {
        let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<$type>()).unwrap();
        let value: $type = <$type>::MAX / 2 as $type;
        let value_bytes = value.to_le_bytes();

        let _ = buffer.write_le(&value);
        let _ = buffer.move_cursor(0);

        assert_eq!(buffer.read::<$type>().unwrap(), value.to_le());

        let _ = buffer.move_cursor(0);
        assert_eq!(buffer.read_le::<$type>().unwrap(), value);

        let _ = buffer.move_cursor(0);

        assert_eq!(
            buffer.read_slice(std::mem::size_of::<$type>()).unwrap(),
            value_bytes
        );
    };
}

macro_rules! test_default_impl_be {
    ($type:ty) => {
        let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<$type>()).unwrap();
        let value: $type = <$type>::MAX / 2 as $type;
        let value_bytes = value.to_be_bytes();

        let _ = buffer.write_be(&value);
        let _ = buffer.move_cursor(0);

        assert_eq!(buffer.read::<$type>().unwrap(), value.to_be());

        let _ = buffer.move_cursor(0);
        assert_eq!(buffer.read_be::<$type>().unwrap(), value);

        let _ = buffer.move_cursor(0);
        assert_eq!(
            buffer.read_slice(std::mem::size_of::<$type>()).unwrap(),
            value_bytes
        );
    };
}

#[test]
fn test_u8_write_read_ne() {
    test_default_impl_ne!(u8);
}

/*
 * No need to test endianness on u8 since it's only a single byte
*/

#[test]
fn test_u16_write_read_ne() {
    test_default_impl_ne!(u16);
}

#[test]
fn test_u16_write_read_le() {
    test_default_impl_le!(u16);
}

#[test]
fn test_u16_write_read_be() {
    test_default_impl_be!(u16);
}

#[test]
fn test_u32_write_read_ne() {
    test_default_impl_ne!(u32);
}

#[test]
fn test_u32_write_read_le() {
    test_default_impl_le!(u32);
}

#[test]
fn test_u32_write_read_be() {
    test_default_impl_be!(u32);
}

#[test]
fn test_u64_write_read_ne() {
    test_default_impl_ne!(u64);
}

#[test]
fn test_u64_write_read_le() {
    test_default_impl_le!(u64);
}

#[test]
fn test_u64_write_read_be() {
    test_default_impl_be!(u64);
}

#[test]
fn test_u128_write_read_ne() {
    test_default_impl_ne!(u128);
}

#[test]
fn test_u128_write_read_le() {
    test_default_impl_le!(u128);
}

#[test]
fn test_u128_write_read_be() {
    test_default_impl_be!(u128);
}

#[test]
fn test_usize_write_read_ne() {
    test_default_impl_ne!(usize);
}

#[test]
fn test_usize_write_read_le() {
    test_default_impl_le!(usize);
}

#[test]
fn test_usize_write_read_be() {
    test_default_impl_be!(usize);
}

#[test]
fn test_i8_write_read_ne() {
    test_default_impl_ne!(i8);
}

/*
 * No need to test endianness on i8 since it's only a single byte
*/

#[test]
fn test_i16_write_read_ne() {
    test_default_impl_ne!(i16);
}

#[test]
fn test_i16_write_read_le() {
    test_default_impl_le!(i16);
}

#[test]
fn test_i16_write_read_be() {
    test_default_impl_be!(i16);
}

#[test]
fn test_i32_write_read_ne() {
    test_default_impl_ne!(i32);
}

#[test]
fn test_i32_write_read_le() {
    test_default_impl_le!(i32);
}

#[test]
fn test_i32_write_read_be() {
    test_default_impl_be!(i32);
}

#[test]
fn test_i64_write_read_ne() {
    test_default_impl_ne!(i64);
}

#[test]
fn test_i64_write_read_le() {
    test_default_impl_le!(i64);
}

#[test]
fn test_i64_write_read_be() {
    test_default_impl_be!(i64);
}

#[test]
fn test_i128_write_read_ne() {
    test_default_impl_ne!(i128);
}

#[test]
fn test_i128_write_read_le() {
    test_default_impl_le!(i128);
}

#[test]
fn test_i128_write_read_be() {
    test_default_impl_be!(i128);
}

#[test]
fn test_isize_write_read_ne() {
    test_default_impl_ne!(isize);
}

#[test]
fn test_isize_write_read_le() {
    test_default_impl_le!(isize);
}

#[test]
fn test_isize_write_read_be() {
    test_default_impl_be!(isize);
}

#[test]
fn test_f32_write_read_ne() {
    test_default_impl_ne!(f32);
}

#[test]
fn test_f32_write_read_le() {
    let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<f32>()).unwrap();
    let value: f32 = f32::MAX / 2.0f32;
    let value_bytes = value.to_le_bytes();

    let _ = buffer.write_le(&value);
    let _ = buffer.move_cursor(0);

    assert_eq!(
        f32::from_le_bytes(buffer.read::<f32>().unwrap().to_ne_bytes()),
        f32::from_le_bytes(value.to_bits().to_le_bytes())
    );

    let _ = buffer.move_cursor(0);
    assert_eq!(buffer.read_le::<f32>().unwrap(), value);

    let _ = buffer.move_cursor(0);

    assert_eq!(
        buffer.read_slice(std::mem::size_of::<f32>()).unwrap(),
        value_bytes
    );
}

#[test]
fn test_f32_write_read_be() {
    let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<f32>()).unwrap();
    let value: f32 = f32::MAX / 2.0f32;
    let value_bytes = value.to_be_bytes();

    let _ = buffer.write_be(&value);
    let _ = buffer.move_cursor(0);

    assert_eq!(
        f32::from_be_bytes(buffer.read::<f32>().unwrap().to_ne_bytes()),
        f32::from_be_bytes(value.to_bits().to_be_bytes())
    );

    let _ = buffer.move_cursor(0);
    assert_eq!(buffer.read_be::<f32>().unwrap(), value);

    let _ = buffer.move_cursor(0);

    assert_eq!(
        buffer.read_slice(std::mem::size_of::<f32>()).unwrap(),
        value_bytes
    );
}

#[test]
fn test_f64_write_read_ne() {
    test_default_impl_ne!(f64);
}

#[test]
fn test_f64_write_read_le() {
    let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<f64>()).unwrap();
    let value: f64 = f64::MAX / 2.0f64;
    let value_bytes = value.to_le_bytes();

    let _ = buffer.write_le(&value);
    let _ = buffer.move_cursor(0);

    assert_eq!(
        f64::from_le_bytes(buffer.read::<f64>().unwrap().to_ne_bytes()),
        f64::from_le_bytes(value.to_bits().to_le_bytes())
    );

    let _ = buffer.move_cursor(0);
    assert_eq!(buffer.read_le::<f64>().unwrap(), value);

    let _ = buffer.move_cursor(0);

    assert_eq!(
        buffer.read_slice(std::mem::size_of::<f64>()).unwrap(),
        value_bytes
    );
}

#[test]
fn test_f64_write_read_be() {
    let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<f64>()).unwrap();
    let value: f64 = f64::MAX / 2.0f64;
    let value_bytes = value.to_be_bytes();

    let _ = buffer.write_be(&value);
    let _ = buffer.move_cursor(0);

    assert_eq!(
        f64::from_be_bytes(buffer.read::<f64>().unwrap().to_ne_bytes()),
        f64::from_be_bytes(value.to_bits().to_be_bytes())
    );

    let _ = buffer.move_cursor(0);
    assert_eq!(buffer.read_be::<f64>().unwrap(), value);

    let _ = buffer.move_cursor(0);

    assert_eq!(
        buffer.read_slice(std::mem::size_of::<f64>()).unwrap(),
        value_bytes
    );
}
