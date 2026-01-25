use crate::bytebuffer::write_macro::write_ty;
use crate::ser_trait::SerBytes;
use byteorder::ByteOrder;
use std::io;
use std::io::ErrorKind;
use std::marker::PhantomData;

pub struct WriteByteBuffer {
    buf: Vec<u8>,
    bit_pos: usize,
}

impl WriteByteBuffer {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            bit_pos: 8,
        }
    }

    pub fn write_bit(&mut self, bit: u8) {
        if self.bit_pos == 8 {
            self.write_u8(0);
            self.bit_pos = 0;
        }

        let len = self.buf.len();

        self.buf[len - 1] |= bit << (7 - self.bit_pos);

        self.bit_pos += 1;
    }

    pub fn write_bits(&mut self, bits: u8, count: usize) {
        let mask = 1;

        for i in 0..count {
            let shifted_right = bits >> count - i;
            let bit = shifted_right & mask;
            self.write_bit(bit);
        }
    }

    pub fn write_remaining_bits(&mut self, bits: u8) -> io::Result<()> {
        if self.bit_pos == 8 {
            return Err(ErrorKind::UnexpectedEof.into());
        }

        let last_bits = self
            .buf
            .last_mut()
            .ok_or_else(|| io::Error::from(ErrorKind::UnexpectedEof))?;

        let bits_masked = bits & (0xFF >> self.bit_pos);
        let bits = *last_bits | bits_masked;

        *last_bits = bits;

        self.bit_pos = 8;

        Ok(())
    }

    pub fn write_bool(&mut self, b: bool) {
        self.write_bit(b as u8);
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes)
    }

    pub fn write_u8(&mut self, n: u8) -> IndexPointer<u8> {
        self.bit_pos = 8;
        let index = self.buf.len();

        self.buf.push(n);

        IndexPointer::new(index, 1)
    }

    pub fn write_i8(&mut self, n: i8) -> IndexPointer<i8> {
        self.bit_pos = 8;
        let index = self.buf.len();

        self.buf.push(n as u8);

        IndexPointer::new(index, 1)
    }

    pub fn write_at_index_pointer<S: SerBytes>(&mut self, index_pointer: &IndexPointer<S>, val: S) {
        let mut temp_bb = WriteByteBuffer::new();
        val.to_buf(&mut temp_bb);

        self.buf[index_pointer.index..(index_pointer.index + index_pointer.len)]
            .copy_from_slice(temp_bb.buf());
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

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.buf.reserve(additional);
    }

    // pub fn flush_bits
}

pub struct IndexPointer<S> {
    index: usize,
    len: usize,
    _s: PhantomData<S>,
}

impl<S> IndexPointer<S> {
    fn new(index: usize, len: usize) -> Self {
        Self {
            index,
            len,
            _s: PhantomData,
        }
    }
}
