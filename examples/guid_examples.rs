use std::io::Cursor;
use winstructs::guid::Guid;

fn main() {
    let guid_buffer_01: &[u8] = &[
        0xEE, 0xE0, 0x9F, 0xF3, 0xDE, 0x74, 0xE1, 0x11, 0xA3, 0xF9, 0x00, 0x50, 0x56, 0xA5, 0x00,
        0x10,
    ];

    let guid1 = Guid::from_stream(&mut Cursor::new(guid_buffer_01)).unwrap();
    println!("{:?}", guid1);

    let guid_buffer_02: &[u8] = &[
        0x4E, 0xEB, 0xCB, 0x79, 0x9D, 0xF2, 0x0C, 0x4A, 0xA7, 0x0E, 0xE5, 0x64, 0x7A, 0x53, 0x97,
        0x0B,
    ];
    let guid2 = Guid::from_stream(&mut Cursor::new(guid_buffer_02)).unwrap();
    println!("{:?}", guid2);
}
