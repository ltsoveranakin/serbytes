use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WithParent, WriteByteBufferOwned};
use crate::ser_bytes_impl::from_buf;
use crate::ser_trait::SerBytes;
use std::time::Duration;

impl SerBytes for Duration {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let mut inner = || {
            let secs = from_buf(buf)?;
            let nanos = from_buf(buf)?;

            Ok(Self::new(secs, nanos))
        };

        inner().with_parent("Duration")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.as_secs().to_buf(buf);
        self.subsec_nanos().to_buf(buf);
    }
}
