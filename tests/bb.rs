use serbytes::prelude::WriteByteBuffer;

fn test_write() {
    let mut wbb = WriteByteBuffer::new();

    let n = 1025;
    wbb.write_u16(n);

    let v = wbb.into_vec();

    println!("{:?}", v);
}
