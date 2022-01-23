use bytey_byte_buffer::byte_buffer::ByteBuffer;
use bytey_byte_buffer::error::ByteBufferError;

#[test]
fn test_min_capacity() {
	let mut buffer = ByteBuffer::with_capacity(0);

	assert!(buffer.is_err());
	assert_eq!(buffer.err().unwrap(), ByteBufferError::MinCapacity);
}

#[test]
fn test_max_capacity() {
	let mut buffer = ByteBuffer::with_capacity(isize::MAX as usize + 1);

	assert!(buffer.is_err());
	assert_eq!(buffer.err().unwrap(), ByteBufferError::MaxCapacity);
}

#[test]
fn test_capacity() {
	let mut buffer = ByteBuffer::with_capacity(4).unwrap();

	assert_eq!(buffer.capacity(), 4);
}

#[test]
fn test_resize_min() {
	let mut buffer = ByteBuffer::with_capacity(4).unwrap();

	assert_eq!(buffer.resize(0).err().unwrap(), ByteBufferError::MinCapacity);
	assert_eq!(buffer.capacity(), 4);
}

#[test]
fn test_resize_max() {
	let mut buffer = ByteBuffer::with_capacity(4).unwrap();

	assert_eq!(buffer.resize(isize::MAX as usize + 1).err().unwrap(), ByteBufferError::MaxCapacity);
	assert_eq!(buffer.capacity(), 4);
}

#[test]
fn test_resize() {
	let mut buffer = ByteBuffer::with_capacity(4).unwrap();

	buffer.resize(1);

	assert_eq!(buffer.capacity(), 1);
}

#[test]
fn test_resize_length_cap() {
	let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<u32>()).unwrap();

	buffer.write(u32::MAX);

	assert_eq!(buffer.length(), 4);
	assert_eq!(buffer.capacity(), 4);

	buffer.resize(2);

	assert_eq!(buffer.length(), 2);
	assert_eq!(buffer.capacity(), 2);
}

#[test]
fn test_resize_cursor_cap() {
	let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<u32>()).unwrap();

	buffer.write(u32::MAX);

	assert_eq!(buffer.cursor(), 4);
	assert_eq!(buffer.capacity(), 4);

	buffer.resize(2);

	assert_eq!(buffer.cursor(), 2);
	assert_eq!(buffer.capacity(), 2);
}

#[test]
fn test_expand_max() {
	let mut buffer = ByteBuffer::new().unwrap();

	assert_eq!(buffer.expand(isize::MAX as usize + 1).err().unwrap(), ByteBufferError::MaxCapacity);
	assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);
}

#[test]
fn test_expand_usize_overflow() {
	let mut buffer = ByteBuffer::new().unwrap();

	assert_eq!(buffer.expand(usize::MAX).err().unwrap(), ByteBufferError::MaxCapacity);
	assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);
}

#[test]
fn test_shrink_min() {
	let mut buffer = ByteBuffer::new().unwrap();

	assert_eq!(buffer.shrink(ByteBuffer::MIN_SIZE).err().unwrap(), ByteBufferError::MinCapacity);
	assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);
}

#[test]
fn test_shrink_usize_underflow() {
	let mut buffer = ByteBuffer::new().unwrap();

	assert_eq!(buffer.shrink(ByteBuffer::MIN_SIZE + 1).err().unwrap(), ByteBufferError::MinCapacity);
	assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);
}

#[test]
fn test_write_slice_capacity() {
	let mut buffer = ByteBuffer::new().unwrap();
	let i: u64 = u64::MAX / 2;
	let i2: u16 = u16::MAX / 2;

	assert_eq!(buffer.capacity(), ByteBuffer::MIN_SIZE);

	buffer.write_slice(&i.to_ne_bytes());
	assert_eq!(buffer.capacity(), 8);

	buffer.write_slice(&i2.to_ne_bytes());
	assert_eq!(buffer.capacity(), 16);
}

#[test]
fn test_write_slice_length() {
	let mut buffer = ByteBuffer::new().unwrap();
	let i: u64 = u64::MAX / 2;
	let i2: u32 = u32::MAX / 2;

	assert_eq!(buffer.length(), 0);

	buffer.write_slice(&i.to_ne_bytes());
	assert_eq!(buffer.length(), 8);

	buffer.move_cursor(6);
	buffer.write_slice(&i2.to_ne_bytes());
	assert_eq!(buffer.length(), 10);
}

#[test]
fn test_write_slice_cursor() {
	let mut buffer = ByteBuffer::new().unwrap();
	let i: u32 = u32::MAX / 2;

	assert_eq!(buffer.cursor(), 0);

	buffer.write_slice(&i.to_ne_bytes());
	assert_eq!(buffer.cursor(), 4);
}

#[test]
fn test_read_slice_bounds() {
	let mut buffer = ByteBuffer::new().unwrap();
	let i: u32 = u32::MAX / 2;

	buffer.write(i);
	buffer.move_cursor(0);

	assert_eq!(buffer.read_slice(5).err().unwrap(), ByteBufferError::ReadOutOfBounds {
		length: 4,
		start: 0,
		end: 5
	});
}

#[test]
fn test_read_slice_cursor() {
	let mut buffer = ByteBuffer::new().unwrap();
	let i: u32 = u32::MAX / 2;

	buffer.write(i);
	buffer.move_cursor(0);
	buffer.read_slice(2);

	assert_eq!(buffer.cursor(), 2);
}