use bincode::{Decode, Encode};
use bytey_bincode::{BincodeDecode, BincodeEncode};
use bytey_byte_buffer::byte_buffer::ByteBuffer;

#[derive(PartialEq, Decode, Encode, Debug)]
struct TestData {
    x: u32,
    y: u64,
    i: f32,
}

#[test]
fn test_struct_writing_reading() {
    let mut buffer = ByteBuffer::with_capacity(std::mem::size_of::<TestData>()).unwrap();
    let value = TestData {
        x: 5,
        y: 10,
        i: 5.5,
    };

    let _ = buffer.encode(&value);
    let _ = buffer.move_cursor(0);
    let new_value: TestData = buffer.decode().unwrap();
    assert_eq!(new_value, value);
}
