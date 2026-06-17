use crate::bytebuffer::ReadByteBufferRefMut;

pub struct ReadByteBufferSlice<'a> {
    pub(super) buf: &'a [u8],
    pub(super) index: usize,
    pub(super) bit_index: usize,
}

impl<'a> ReadByteBufferSlice<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            buf,
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
}
