pub type Result<T> = std::result::Result<T, ByteBufferError>;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum ByteBufferError {
	#[error("Capacity cannot be greater than {} bytes", isize::MAX)]
	MaxCapacity,

	#[error("Failed to allocate {size} bytes")]
	AllocationFailure {
		size: usize
	},

	#[error("Capacity cannot be less than 1 byte")]
	MinCapacity,

	#[error("Cursor out of bounds: {cursor} >= {length}")]
	CursorOutOfBounds {
		length: usize,
		cursor: usize
	},

	#[error("Read out of bounds: {start}..{end} >= {length}")]
	ReadOutOfBounds {
		length: usize,
		start: usize,
		end: usize
	}
}