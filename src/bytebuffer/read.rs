use crate::bytebuffer::read_macro::read_ty;
use crate::prelude::WriteByteBuffer;
use byteorder::ByteOrder;
use std::io::ErrorKind;
use std::{io, result};

#[derive(Debug)]
pub struct ReadError {
    pub(crate) message: String,
}

pub type Result<T> = result::Result<T, ReadError>;

impl From<ReadError> for io::Error {
    fn from(_: ReadError) -> Self {
        ErrorKind::UnexpectedEof.into()
    }
}

pub struct ReadByteBuffer {
    buf: Vec<u8>,
    index: usize,
    bit_pos: usize,
}

impl ReadByteBuffer {
    pub fn new() -> Self {
        Self::from_vec(vec![])
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_vec(bytes.to_vec())
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            buf: vec,
            index: 0,
            bit_pos: 8,
        }
    }

    fn has_bytes_remaining(&self, remaining: usize) -> bool {
        self.buf.len() - self.index >= remaining
    }

    pub fn read_bit(&mut self) -> Result<u8> {
        if self.bit_pos == 8 {
            self.index += 1;
            self.bit_pos = 0;
        }

        let bit = self.buf[self.index] >> (7 - self.bit_pos) & 1;

        self.bit_pos += 1;

        Ok(bit)
    }

    pub fn read_remaining_bits(&mut self) -> Result<u8> {
        if self.bit_pos > 7 {
            return Err(ReadError {
                message: "remaining bits".into(),
            });
        }

        let shifted_left = self.buf[self.index] << self.bit_pos;
        let bits_shifted = shifted_left & (!0);
        let bits = bits_shifted >> self.bit_pos;

        self.index += 1;

        self.bit_pos = 8;

        Ok(bits)
    }

    pub(crate) fn read_bytes_err_msg(&mut self, size: usize, message: String) -> Result<&[u8]> {
        let index = self.index;
        if !self.has_bytes_remaining(size) {
            Err(ReadError { message })
        } else {
            self.index += size;
            self.bit_pos = 8;

            let slice = &self.buf[index..index + size];
            Ok(slice)
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> Result<&[u8]> {
        self.read_bytes_err_msg(size, format!("bytes of size {}", size))
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let byte = self.buf.get(self.index).ok_or_else(|| ReadError {
            message: "u8".into(),
        })?;

        self.index += 1;
        self.bit_pos = 8;

        Ok(*byte)
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        self.read_u8().map(|u| u as i8)
    }

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

impl From<WriteByteBuffer> for ReadByteBuffer {
    fn from(value: WriteByteBuffer) -> Self {
        Self::from_vec(value.into_vec())
    }
}
