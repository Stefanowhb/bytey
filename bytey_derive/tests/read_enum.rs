use bytey::ByteBuffer;
use bytey_derive::{ByteBufferRead, ByteBufferWrite};

#[test]
fn test_enum_read() {
    #[derive(ByteBufferWrite, ByteBufferRead, PartialEq, Debug)]
    enum Test {
        Named { a: u16, b: u8, c: i64, d: usize },
        Unnamed(u16, u8, i64, usize),
        Unit,
    }

    let mut buffer = ByteBuffer::new().unwrap();
    let val_named = Test::Named {
        a: 128,
        b: 255,
        c: -255,
        d: usize::MAX,
    };

    buffer.write(&val_named).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(val_named, buffer.read::<Test>().unwrap());

    let mut buffer = ByteBuffer::new().unwrap();
    let val_unnamed = Test::Unnamed(128, 255, -255, usize::MAX);

    buffer.write(&val_unnamed).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(val_unnamed, buffer.read::<Test>().unwrap());

    let mut buffer = ByteBuffer::new().unwrap();
    let val_unit = Test::Unit;

    buffer.write(&val_unit).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(val_unit, buffer.read::<Test>().unwrap());
}
