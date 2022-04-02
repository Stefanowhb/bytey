use bytey::ByteBuffer;
use bytey_derive::{ByteBufferRead, ByteBufferWrite};

#[test]
fn test_struct_named_read() {
    #[derive(ByteBufferWrite, ByteBufferRead, PartialEq, Debug)]
    struct Test {
        a: u16,
        b: u8,
        c: i64,
        d: usize,
    }

    let mut buffer = ByteBuffer::new().unwrap();
    let val = Test {
        a: 128,
        b: 255,
        c: -255,
        d: usize::MAX,
    };

    buffer.write(&val);
    buffer.move_cursor(0);

    assert_eq!(val, buffer.read::<Test>().unwrap());
}

#[test]
fn test_struct_unnamed_read() {
    #[derive(ByteBufferWrite, ByteBufferRead, PartialEq, Debug)]
    struct Test(u16, u8, i64, usize);

    let mut buffer = ByteBuffer::new().unwrap();
    let val = Test(128, 255, -255, usize::MAX);

    buffer.write(&val);
    buffer.move_cursor(0);

    assert_eq!(val, buffer.read::<Test>().unwrap());
}
