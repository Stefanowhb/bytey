use bytey::ByteBuffer;
use bytey_derive::{ByteBufferRead, ByteBufferWrite};

#[test]
fn test_struct_skip() {
    #[derive(ByteBufferWrite, ByteBufferRead, PartialEq, Debug)]
    struct Test {
        a: u16,
        b: u8,
        #[bytey(skip)]
        c: i64,
        d: usize,
    }

    let mut buffer = ByteBuffer::new().unwrap();
    let mut val = Test {
        a: 128,
        b: 255,
        c: -255,
        d: usize::MAX,
    };

    buffer.write(&val).unwrap();
    buffer.move_cursor(0).unwrap();

    val.c = 0;
    assert_eq!(val, buffer.read::<Test>().unwrap());
}

#[test]
fn test_struct_unnamed_skip() {
    #[derive(ByteBufferWrite, ByteBufferRead, PartialEq, Debug)]
    struct Test(u16, u8, #[bytey(skip)] i64, usize);

    let mut buffer = ByteBuffer::new().unwrap();
    let val = Test(128, 255, -255, usize::MAX);

    buffer.write(&val).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_ne!(val, buffer.read::<Test>().unwrap());
}
