use serbytes::prelude::{ReadByteBuffer, WriteByteBuffer};

#[test]
fn test_individual_bits() {
    let mut wbb = WriteByteBuffer::new();

    let b1 = true;
    let b2 = false;
    let b3 = false;
    let b4 = true;
    let b5 = true;
    let b6 = true;
    let b7 = false;
    let b8 = false;
    let b1b = true;

    let byte = 143;

    let b1c = true;

    wbb.write_bool(b1);
    wbb.write_bool(b2);
    wbb.write_bool(b3);
    wbb.write_bool(b4);
    wbb.write_bool(b5);
    wbb.write_bool(b6);
    wbb.write_bool(b7);
    wbb.write_bool(b8);
    wbb.write_bool(b1b);

    wbb.write_u8(byte);

    wbb.write_bool(b1c);

    let v = wbb.into_vec();

    let mut rbb = ReadByteBuffer::from_vec(v);

    assert_eq!(rbb.read_bool().expect("Bit 1 to exist"), b1);
    assert_eq!(rbb.read_bool().expect("Bit 2 to exist"), b2);
    assert_eq!(rbb.read_bool().expect("Bit 3 to exist"), b3);
    assert_eq!(rbb.read_bool().expect("Bit 4 to exist"), b4);
    assert_eq!(rbb.read_bool().expect("Bit 5 to exist"), b5);
    assert_eq!(rbb.read_bool().expect("Bit 6 to exist"), b6);
    assert_eq!(rbb.read_bool().expect("Bit 7 to exist"), b7);
    assert_eq!(rbb.read_bool().expect("Bit 8 to exist"), b8);
    assert_eq!(rbb.read_bool().expect("Bit 1 post bits to exist"), b1b);

    assert_eq!(rbb.read_u8().expect("Byte to exist"), byte);

    assert_eq!(
        rbb.read_bool()
            .expect("Bit 1 post whole byte write to exist"),
        b1c
    );
}

#[test]
fn test_rem_bits() {
    let mut wbb = WriteByteBuffer::new();

    let b1 = true;
    let b2 = false;
    let b3 = false;
    let b4 = true;
    let b5 = true;

    let rem = 6;

    wbb.write_bool(b1);
    wbb.write_bool(b2);
    wbb.write_bool(b3);
    wbb.write_bool(b4);
    wbb.write_bool(b5);
    wbb.write_remaining_bits(rem)
        .expect("Bits remaining to write to");

    let v = wbb.into_vec();

    let mut rbb = ReadByteBuffer::from_vec(v);

    assert_eq!(rbb.read_bool().expect("Bit 1 to exist"), b1);
    assert_eq!(rbb.read_bool().expect("Bit 2 to exist"), b2);
    assert_eq!(rbb.read_bool().expect("Bit 3 to exist"), b3);
    assert_eq!(rbb.read_bool().expect("Bit 4 to exist"), b4);
    assert_eq!(rbb.read_bool().expect("Bit 5 to exist"), b5);
    assert_eq!(
        rbb.read_remaining_bits().expect("Remaining bits to exist"),
        rem
    );
}

#[test]
fn test_write_bits() {
    let mut wbb = WriteByteBuffer::new();

    let bits = 110;

    wbb.write_bits(bits, 7);

    let mut rbb = ReadByteBuffer::from_vec(wbb.into_vec());

    assert_eq!(
        rbb.read_bits(7).expect("7 Bits to be able to be read"),
        bits
    );
}
