use bytey::ByteBuffer;
use bytey_derive::ByteBufferWrite;

#[test]
fn test_enum_write() {
    #[derive(ByteBufferWrite)]
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

    buffer.write(val_named).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(
        [
            1, 0, /* u16 for variant named id: 1 */
            128, 0,   /* u16: 128 */
            255, /* u8: 255 */
            1, 255, 255, 255, 255, 255, 255, 255, /* i64: -255 */
            255, 255, 255, 255, 255, 255, 255, 255 /* usize: usize::MAX */
        ],
        buffer.read_slice(21).unwrap()
    );

    let mut buffer = ByteBuffer::new().unwrap();
    let val_unnamed = Test::Unnamed(128, 255, -255, usize::MAX);

    buffer.write(val_unnamed).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(
        [
            2, 0, /* u16 for variant unnamed id: 2 */
            128, 0,   /* u16: 128 */
            255, /* u8: 255 */
            1, 255, 255, 255, 255, 255, 255, 255, /* i64: -255 */
            255, 255, 255, 255, 255, 255, 255, 255 /* usize: usize::MAX */
        ],
        buffer.read_slice(21).unwrap()
    );

    let mut buffer = ByteBuffer::new().unwrap();
    let val_unit = Test::Unit;

    buffer.write(val_unit).unwrap();
    buffer.move_cursor(0).unwrap();

    assert_eq!(
        [3, 0], /* u16 for variant unit id: 3 */
        buffer.read_slice(2).unwrap()
    );
}
