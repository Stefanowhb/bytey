use bytey_byte_buffer::byte_buffer::ByteBuffer;
use bytey_derive::{ByteBufferRead, ByteBufferWrite};

fn test_enum_read() {
    #[derive(ByteBufferWrite, ByteBufferRead, PartialEq, Debug)]
    enum Test {
        VariantNamed { a: u16, b: u8, c: i64, d: usize },
        VariantUnnamed(u16, u8, i64, usize),
        VariantUnit,
    }

    let mut buffer = ByteBuffer::new().unwrap();
    let val_named = Test::VariantNamed {
        a: 128,
        b: 255,
        c: -255,
        d: usize::MAX,
    };

    buffer.write(&val_named);
    buffer.move_cursor(0);

    assert_eq!(val_named, buffer.read::<Test>().unwrap());

    let mut buffer = ByteBuffer::new().unwrap();
    let val_unnamed = Test::VariantUnnamed(128, 255, -255, usize::MAX);

    buffer.write(&val_unnamed);
    buffer.move_cursor(0);

    assert_eq!(val_unnamed, buffer.read::<Test>().unwrap());

    let mut buffer = ByteBuffer::new().unwrap();
    let val_unit = Test::VariantUnit;

    buffer.write(&val_unit);
    buffer.move_cursor(0);

    assert_eq!(val_unit, buffer.read::<Test>().unwrap());
}
