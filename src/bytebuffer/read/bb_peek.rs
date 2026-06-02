use crate::bytebuffer::ReadByteBufferRefMut;

pub struct ReadByteBufferPeek<'a> {
    pub(super) buf: &'a [u8],
    pub(super) index: usize,
    pub(super) bit_index: usize,
}

impl<'a> ReadByteBufferPeek<'a> {
    pub fn rbb_ref_mut(&mut self) -> ReadByteBufferRefMut<'_> {
        ReadByteBufferRefMut {
            buf: &self.buf,
            index: &mut self.index,
            bit_index: &mut self.bit_index,
        }
    }
}
