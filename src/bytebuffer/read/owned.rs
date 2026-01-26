use crate::bytebuffer::read::read_macro::read_owned_ty;
use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};

pub struct ReadByteBufferOwned {
    buf: Vec<u8>,
    index: usize,
    bit_index: usize,
}

impl ReadByteBufferOwned {
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            buf: vec,
            index: 0,
            bit_index: 0,
        }
    }

    pub fn rbb_ref_mut(&mut self) -> ReadByteBufferRefMut<'_> {
        ReadByteBufferRefMut {
            buf: &self.buf,
            index: &mut self.index,
            bit_index: &mut self.bit_index,
        }
    }

    pub fn read_bit(&mut self) -> BBReadResult<u8> {
        self.rbb_ref_mut().read_bit()
    }

    pub fn read_bits(&mut self, count: usize) -> BBReadResult<u8> {
        self.rbb_ref_mut().read_bits(count)
    }

    /// Reads `count` bytes to a vector.
    /// If you would like to avoid the extra overhead of a vec, use [`ReadByteBufferOwned::rbb_ref_mut`]
    /// to obtain a [`ReadByteBufferRefMut`], on which you can call [`ReadByteBufferRefMut::read_bytes`]

    pub fn read_bytes_to_vec(&mut self, count: usize) -> BBReadResult<Vec<u8>> {
        let mut rbb_ref = self.rbb_ref_mut();
        let bytes =
            rbb_ref.read_bytes_with_err_msg(count, format!("bytes to vec; len: {}", count))?;

        Ok(bytes.to_vec())
    }

    // pub fn read_bits(&mut self, count: usize) -> BBReadResult<u8> {
    //     let final_bit_index = self.bit_index + count - 1;
    //     if final_bit_index > 7 {
    //         return Err(ReadError::new(format!(
    //             "read bits; count: {}; bit_index: {}",
    //             count, self.bit_index
    //         )));
    //     }
    //
    //     let read_bits = self.buf[self.index];
    //     let bits_shifted_l = read_bits << self.bit_index;
    //     let bits = bits_shifted_l >> ((7 - final_bit_index) + self.bit_index);
    //
    //     self.bit_index = final_bit_index + 1;
    //
    //     if self.bit_index == 8 {
    //         self.flush_bit_index();
    //     }
    //
    //     Ok(bits)
    // }

    pub fn read_remaining_bits(&mut self) -> BBReadResult<u8> {
        self.rbb_ref_mut().read_remaining_bits()
    }

    pub fn read_bool(&mut self) -> BBReadResult<bool> {
        self.rbb_ref_mut().read_bool()
    }

    pub fn read_u8(&mut self) -> BBReadResult<u8> {
        self.rbb_ref_mut().read_u8()
    }

    pub fn read_i8(&mut self) -> BBReadResult<i8> {
        self.rbb_ref_mut().read_i8()
    }

    read_owned_ty!(u16, read_u16, 2);
    read_owned_ty!(u32, read_u32, 4);
    read_owned_ty!(u64, read_u64, 8);
    read_owned_ty!(u128, read_u128, 16);

    read_owned_ty!(i16, read_i16, 2);
    read_owned_ty!(i32, read_i32, 4);
    read_owned_ty!(i64, read_i64, 8);
    read_owned_ty!(i128, read_i128, 16);

    read_owned_ty!(f32, read_f32, 4);
    read_owned_ty!(f64, read_f64, 8);

    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }

    pub fn flush_bits(&mut self) {
        if self.bit_index != 0 {
            self.index += 1;
        }
        self.bit_index = 0;
    }
}

impl From<WriteByteBufferOwned> for ReadByteBufferOwned {
    fn from(value: WriteByteBufferOwned) -> Self {
        Self::from_vec(value.into_vec())
    }
}
