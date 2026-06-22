use crate::bytebuffer;
use crate::bytebuffer::{
    ReadByteBufferRefMut, ReadError, SpecificError, WithParent, WriteByteBufferOwned,
};
use crate::prelude::SerBytes;
use crate::ser_bytes_impl::U8Vec;

impl SerBytes for String {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> bytebuffer::BBReadResult<Self> {
        let mut inner = || {
            let u8_vec = U8Vec::<u16>::from_buf(buf)?;

            String::from_utf8(u8_vec.vec).map_err(|_| {
                ReadError::new(
                    SpecificError::Other("Invalid utf8".into()),
                    "Validate utf8",
                    None,
                )
            })
        };

        inner().with_parent("String")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        buf.reserve(self.approx_size());

        buf.write_u16(self.len() as u16);
        buf.write_bytes(self.as_bytes());
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        u16::size_hint()
    }

    fn approx_size(&self) -> usize {
        u16::size_hint() + self.len()
    }
}
