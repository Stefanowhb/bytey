use bytey_byte_buffer::byte_buffer::ByteBuffer;
use bytey_byte_buffer::error::ByteBufferError;

#[test]
fn test_min_capacity() {
    let buffer = ByteBuffer::with_capacity(0);

    assert!(buffer.is_err());
    assert_eq!(buffer.err().unwrap(), ByteBufferError::MinCapacity);
}

#[test]
fn test_max_capacity() {
    let buffer = ByteBuffer::with_capacity(isize::MAX as usize + 1);

    assert!(buffer.is_err());
    assert_eq!(buffer.err().unwrap(), ByteBufferError::MaxCapacity);
}

#[test]
fn test_capacity() {
    let buffer = ByteBuffer::with_capacity(4).unwrap();

    assert_eq!(buffer.capacity(), 4);
}

#[test]
fn test_resize_min() {
    let mut buffer = ByteBuffer::with_capacity(4).unwrap();

    assert_eq!(
        buffer.resize(0).err().unwrap(),
        ByteBufferError::MinCapacity
    );
    assert_eq!(buffer.capacity(), 4);
}

#[test]
fn test_resize_max() {
    let mut buffer = ByteBuffer::with_capacity(4).unwrap();

    assert_eq!(
        buffer.resize(isize::MAX as usize + 1).err().unwrap(),
        ByteBufferError::MaxCapacity
    );
    assert_eq!(buffer.capacity(), 4);
}

#[test]
fn test_resize() {
    let mut buffer = ByteBuffer::with_capacity(4).unwrap();

    let _ = buffer.resize(1);

    assert_eq!(buffer.capacity(), 1);
}

#[test]
fn test_resize_length_cap() {
    let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<u32>()).unwrap();

    let _ = buffer.write(u32::MAX);

    assert_eq!(buffer.length(), 4);
    assert_eq!(buffer.capacity(), 4);

    let _ = buffer.resize(2);

    assert_eq!(buffer.length(), 2);
    assert_eq!(buffer.capacity(), 2);
}

#[test]
fn test_resize_cursor_cap() {
    let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<u32>()).unwrap();

    let _ = buffer.write(u32::MAX);

    assert_eq!(buffer.cursor(), 4);
    assert_eq!(buffer.capacity(), 4);

    let _ = buffer.resize(2);

    assert_eq!(buffer.cursor(), 2);
    assert_eq!(buffer.capacity(), 2);
}

#[test]
fn test_expand_max() {
    let mut buffer = ByteBuffer::new().unwrap();

    assert_eq!(
        buffer.expand(isize::MAX as usize + 1).err().unwrap(),
        ByteBufferError::MaxCapacity
    );
    assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);
}

#[test]
fn test_expand_usize_overflow() {
    let mut buffer = ByteBuffer::new().unwrap();

    assert_eq!(
        buffer.expand(usize::MAX).err().unwrap(),
        ByteBufferError::MaxCapacity
    );
    assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);
}

#[test]
fn test_shrink_to_fit_min() {
    let mut buffer = ByteBuffer::new().unwrap();

    assert_eq!(
        buffer.shrink_to_fit().err().unwrap(),
        ByteBufferError::MinCapacity
    );
    assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);
}

#[test]
fn test_shrink_to() {
    let mut buffer = ByteBuffer::new().unwrap();

    assert_eq!(
        buffer.shrink_to(0).err().unwrap(),
        ByteBufferError::MinCapacity
    );
    assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);
}

#[test]
fn test_write_slice_capacity() {
    let mut buffer = ByteBuffer::new().unwrap();
    let i: u64 = u64::MAX / 2;
    let i2: u16 = u16::MAX / 2;

    assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);

    let _ = buffer.write_slice(&i.to_ne_bytes());
    assert_eq!(buffer.capacity(), 8);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.capacity(), 16);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.capacity(), 16);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.capacity(), 16);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.capacity(), 16);

    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.capacity(), 32);
}

#[test]
fn test_write_slice_length() {
    let mut buffer = ByteBuffer::new().unwrap();
    let i: u64 = u64::MAX / 2;
    let i2: u32 = u32::MAX / 2;

    assert_eq!(buffer.length(), 0);

    let _ = buffer.write_slice(&i.to_ne_bytes());
    assert_eq!(buffer.length(), 8);

    let _ = buffer.move_cursor(6);
    let _ = buffer.write_slice(&i2.to_ne_bytes());
    assert_eq!(buffer.length(), 10);
}

#[test]
fn test_write_slice_cursor() {
    let mut buffer = ByteBuffer::new().unwrap();
    let i: u32 = u32::MAX / 2;

    assert_eq!(buffer.cursor(), 0);

    let _ = buffer.write_slice(&i.to_ne_bytes());
    assert_eq!(buffer.cursor(), 4);
}

#[test]
fn test_read_slice_bounds() {
    let mut buffer = ByteBuffer::new().unwrap();
    let i: u32 = u32::MAX / 2;

    let _ = buffer.write(i);
    let _ = buffer.move_cursor(0);

    assert_eq!(
        buffer.read_slice(5).err().unwrap(),
        ByteBufferError::ReadOutOfBounds {
            length: 4,
            start: 0,
            end: 5
        }
    );
}

#[test]
fn test_read_slice_cursor() {
    let mut buffer = ByteBuffer::new().unwrap();
    let i: u32 = u32::MAX / 2;

    let _ = buffer.write(i);
    let _ = buffer.move_cursor(0);
    let _ = buffer.read_slice(2);

    assert_eq!(buffer.cursor(), 2);
}

#[test]
fn test_read_to_buffer() {
    let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<u64>()).unwrap();
    let value: u64 = 64;

    let _ = buffer.write(value);
    let _ = buffer.move_cursor(0);

    let mut new_buffer = buffer.read_to_buffer(std::mem::size_of::<u64>()).unwrap();
    let read_arr = new_buffer.read::<u64>().unwrap();

    assert_eq!(read_arr, value);
    assert_eq!(buffer.cursor(), 8);
    assert_eq!(new_buffer.capacity(), buffer.capacity());
}
