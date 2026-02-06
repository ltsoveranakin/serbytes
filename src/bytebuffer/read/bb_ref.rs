use crate::bytebuffer::read::read_macro::read_ref_ty;
use crate::bytebuffer::{BBReadResult, ReadError};

pub struct ReadByteBufferRefMut<'a> {
    pub(super) buf: &'a [u8],
    pub(super) index: &'a mut usize,
    pub(super) bit_index: &'a mut usize,
}

impl<'a> ReadByteBufferRefMut<'a> {
    pub fn from_bytes(buf: &'a [u8], index: &'a mut usize, bit_index: &'a mut usize) -> Self {
        Self {
            buf,
            index,
            bit_index,
        }
    }

    pub fn read_bit(&mut self) -> BBReadResult<u8> {
        let bit = self.buf[*self.index] >> (7 - *self.bit_index) & 1;

        *self.bit_index += 1;

        if *self.bit_index == 8 {
            self.flush_bits();
        }

        Ok(bit)
    }

    pub fn flush_bits(&mut self) {
        if *self.bit_index != 0 {
            *self.index += 1;
        }
        *self.bit_index = 0;
    }

    pub fn read_bits(&mut self, count: usize) -> BBReadResult<u8> {
        let mut bits = 0;

        for i in 0..count {
            let bit = self.read_bit()?;
            let shifted = bit << (count - i);

            bits |= shifted;
        }

        Ok(bits)
    }

    pub fn read_remaining_bits(&mut self) -> BBReadResult<u8> {
        if *self.bit_index == 8 {
            return Err(ReadError::new("remaining bits".into()));
        }

        let mask = 0xFF >> *self.bit_index;
        let read_bits = self.buf[*self.index];
        let bits = read_bits & mask;

        self.flush_bits();

        Ok(bits)
    }

    pub fn read_bool(&mut self) -> BBReadResult<bool> {
        Ok(self.read_bit()? == 1)
    }

    pub(crate) fn read_bytes_with_err_msg(
        &mut self,
        size: usize,
        message: String,
    ) -> BBReadResult<&[u8]> {
        if !self.has_bytes_remaining(size) {
            Err(ReadError::new(format!("Error reading: {}", message)))
        } else {
            self.flush_bits();
            let index = *self.index;

            *self.index += size;

            let slice = &self.buf[index..(index + size)];
            Ok(slice)
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> BBReadResult<&[u8]> {
        self.read_bytes_with_err_msg(size, format!("bytes of size {}", size))
    }

    pub fn read_u8(&mut self) -> BBReadResult<u8> {
        self.flush_bits();
        let byte = *self.buf.get(*self.index).ok_or_else(|| ReadError {
            message: "u8".into(),
        })?;

        *self.index += 1;

        Ok(byte)
    }

    pub fn read_i8(&mut self) -> BBReadResult<i8> {
        self.read_u8().map(|u| u as i8)
    }

    read_ref_ty!(u16, read_u16, 2);
    read_ref_ty!(u32, read_u32, 4);
    read_ref_ty!(u64, read_u64, 8);
    read_ref_ty!(u128, read_u128, 16);

    read_ref_ty!(i16, read_i16, 2);
    read_ref_ty!(i32, read_i32, 4);
    read_ref_ty!(i64, read_i64, 8);
    read_ref_ty!(i128, read_i128, 16);

    read_ref_ty!(f32, read_f32, 4);
    read_ref_ty!(f64, read_f64, 8);

    pub(super) fn has_bytes_remaining(&self, remaining: usize) -> bool {
        self.buf.len() - *self.index >= remaining
    }
}
