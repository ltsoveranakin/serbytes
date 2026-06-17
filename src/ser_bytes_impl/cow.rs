use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::borrow::Cow;

impl<'a, S, B> SerBytes for Cow<'a, B>
where
    B: ToOwned<Owned = S>,
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(Cow::Owned(S::from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.clone().into_owned().to_buf(buf)
    }
}
