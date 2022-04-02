use bytey::ByteBuffer;
use bytey_derive::ByteBufferWrite;

fn test_enum_write() {
    #[derive(ByteBufferWrite)]
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

    buffer.write(val_named);
    buffer.move_cursor(0);

    assert_eq!(
        [
            0, 1, /* u16 for variant named id: 1 */
            128, 0,   /* u16: 128 */
            255, /* u8: 255 */
            1, 255, 255, 255, 255, 255, 255, 255, /* i64: -255 */
            255, 255, 255, 255, 255, 255, 255, 255 /* usize: usize::MAX */
        ],
        buffer.read_slice(21).unwrap()
    );

    let mut buffer = ByteBuffer::new().unwrap();
    let val_unnamed = Test::VariantUnnamed(128, 255, -255, usize::MAX);

    buffer.write(val_unnamed);
    buffer.move_cursor(0);

    assert_eq!(
        [
            0, 2, /* u16 for variant unnamed id: 2 */
            128, 0,   /* u16: 128 */
            255, /* u8: 255 */
            1, 255, 255, 255, 255, 255, 255, 255, /* i64: -255 */
            255, 255, 255, 255, 255, 255, 255, 255 /* usize: usize::MAX */
        ],
        buffer.read_slice(21).unwrap()
    );

    let mut buffer = ByteBuffer::new().unwrap();
    let val_unit = Test::VariantUnit;

    buffer.write(val_unit);
    buffer.move_cursor(0);

    assert_eq!(
        [0, 3], /* u16 for variant unit id: 3 */
        buffer.read_slice(2).unwrap()
    );
}
