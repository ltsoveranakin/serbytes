use crate::bytebuffer;
use crate::bytebuffer::{ReadByteBufferRefMut, ReadError, WithParent, WriteByteBufferOwned};
use crate::prelude::{SerBytes, SpecificError};
pub mod hashmap;
pub mod vec_like;

pub use vec_like::*;

impl SerBytes for String {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> bytebuffer::BBReadResult<Self> {
        let mut inner = || {
            let len = u16::from_buf(buf)? as usize;
            let bytes = buf.read_bytes(len)?;

            String::from_utf8(bytes.to_vec()).map_err(|_| {
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
