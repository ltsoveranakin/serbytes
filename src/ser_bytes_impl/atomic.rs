use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::sync::atomic::{AtomicU64, Ordering};

impl SerBytes for AtomicU64 {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(buf.read_u64()?.into())
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.load(Ordering::Relaxed).to_buf(buf);
    }
}
