use crate::bytebuffer::write_macro::write_ty;
use byteorder::{BigEndian, ByteOrder};
use std::io;
use std::io::ErrorKind;

pub struct WriteByteBuffer {
    buf: Vec<u8>,
    bit_pos: usize,
}

impl WriteByteBuffer {
    pub fn new() -> Self {
        Self {
            buf: vec![],
            bit_pos: 8,
        }
    }

    pub fn write_bit(&mut self, bit: bool) {
        if self.bit_pos == 8 {
            self.write_u8(0);
            self.bit_pos = 0;
        }

        let len = self.buf.len();

        self.buf[len - 1] |= (bit as u8) << (7 - self.bit_pos);

        self.bit_pos += 1;
    }

    pub fn write_remaining_bits(&mut self, bits: u8) -> io::Result<u8> {
        if self.bit_pos == 8 {
            return Err(ErrorKind::UnexpectedEof.into());
        }

        let shifted_left = self.buf.last().unwrap() << self.bit_pos;
        let bits_shifted = shifted_left & (!0);
        let bits = bits_shifted >> self.bit_pos;

        self.bit_pos = 8;

        Ok(bits)
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes)
    }

    pub fn write_u8(&mut self, n: u8) {
        self.bit_pos = 8;
        self.buf.push(n);
    }

    pub fn write_i8(&mut self, n: i8) {
        self.write_u8(n as u8);
    }

    write_ty!(u16, write_u16, 2);
    write_ty!(u32, write_u32, 4);
    write_ty!(u64, write_u64, 8);
    write_ty!(u128, write_u128, 16);

    write_ty!(i16, write_i16, 2);
    write_ty!(i32, write_i32, 4);
    write_ty!(i64, write_i64, 8);
    write_ty!(i128, write_i128, 16);

    write_ty!(f32, write_f32, 4);
    write_ty!(f64, write_f64, 8);

    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }

    pub fn buf(&self) -> &Vec<u8> {
        &self.buf
    }
}
