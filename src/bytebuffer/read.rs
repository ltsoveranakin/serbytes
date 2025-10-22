use crate::bytebuffer::read_macro::read_ty;
use byteorder::{BigEndian, ByteOrder};
use std::io;

pub struct ReadByteBuffer {
    buf: Vec<u8>,
    index: usize,
}

impl ReadByteBuffer {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            buf: bytes.to_vec(),
            index: 0,
        }
    }

    pub fn new() -> Self {
        Self {
            buf: vec![],
            index: 0,
        }
    }

    fn has_bytes_remaining(&self, remaining: usize) -> bool {
        self.buf.len() - self.index >= remaining
    }

    pub fn read_u8(&mut self) -> io::Result<u8> {
        let byte = self.buf.get(self.index);

        self.index += 1;

        byte.copied()
    }

    pub fn read_i8(&mut self) -> io::Result<i8> {
        self.read_u8().map(|u| u as i8)
    }

    read_ty!(u8, read_u8, 2);
    read_ty!(u16, read_u16, 2);
    read_ty!(u32, read_u32, 4);
    read_ty!(u64, read_u64, 8);
    read_ty!(u128, read_u128, 16);

    read_ty!(i16, read_i16, 2);
    read_ty!(i32, read_i32, 4);
    read_ty!(i64, read_i64, 8);
    read_ty!(i128, read_i128, 16);

    read_ty!(f32, read_f32, 4);
    read_ty!(f64, read_f64, 8);

    pub(crate) fn into_vec(self) -> Vec<u8> {
        self.buf
    }
}
