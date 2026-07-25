use crate::bytebuffer::read::read_macro::read_owned_ty;
use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::prelude::ReadByteBufferSlice;

pub struct ReadByteBufferOwned {
    buf: Vec<u8>,
    index: usize,
    bit_index: usize,
}

impl ReadByteBufferOwned {
    pub fn from_vec<V>(vec: V) -> Self
    where
        V: Into<Vec<u8>>,
    {
        Self {
            buf: vec.into(),
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

    pub fn peek(&self) -> ReadByteBufferSlice<'_> {
        ReadByteBufferSlice {
            buf: &self.buf,
            index: self.index,
            bit_index: self.bit_index,
        }
    }

    /// Reads `size` bytes to a vector.
    /// If you would like to avoid the extra overhead of a vec, use [`ReadByteBufferOwned::rbb_ref_mut`]
    /// to obtain a [`ReadByteBufferRefMut`], on which you can call [`ReadByteBufferRefMut::read_bytes`]

    pub fn read_bytes_to_vec(&mut self, size: usize) -> BBReadResult<Vec<u8>> {
        let mut rbb_ref = self.rbb_ref_mut();
        let bytes = rbb_ref.read_bytes(size)?;

        Ok(bytes.to_vec())
    }

    /// Shorthand for [`ReadByteBufferRefMut::read_bits`]

    pub fn read_bits(&mut self, count: usize) -> BBReadResult<u64> {
        self.rbb_ref_mut().read_bits(count)
    }

    read_owned_ty!(u8, read_bit);
    read_owned_ty!(bool, read_bool);

    read_owned_ty!((usize, u8), read_remaining_bits);

    read_owned_ty!(u8, read_u8);
    read_owned_ty!(u16, read_u16);
    read_owned_ty!(u32, read_u32);
    read_owned_ty!(u64, read_u64);
    read_owned_ty!(u128, read_u128);

    read_owned_ty!(i8, read_i8);
    read_owned_ty!(i16, read_i16);
    read_owned_ty!(i32, read_i32);
    read_owned_ty!(i64, read_i64);
    read_owned_ty!(i128, read_i128);

    read_owned_ty!(f32, read_f32);
    read_owned_ty!(f64, read_f64);

    /// Shorthand for [`ReadByteBufferRefMut::flush_bits`]

    pub fn flush_bits(&mut self) {
        self.rbb_ref_mut().flush_bits();
    }

    pub fn buf(&self) -> &Vec<u8> {
        &self.buf
    }

    pub fn reset(mut self) -> Self {
        self.bit_index = 0;
        self.index = 0;
        self
    }
}

impl From<WriteByteBufferOwned> for ReadByteBufferOwned {
    fn from(value: WriteByteBufferOwned) -> Self {
        Self::from_vec(value)
    }
}

impl From<ReadByteBufferOwned> for Vec<u8> {
    fn from(value: ReadByteBufferOwned) -> Self {
        value.buf
    }
}
