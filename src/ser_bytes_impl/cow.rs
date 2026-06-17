use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::borrow::Cow;

impl<'a, S, B> SerBytes for Cow<'a, B>
where
    B: ToOwned<Owned = S> + ?Sized,
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

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }

    // While we could convert the value to owned to find it, this can be an expensive operation.
    // So just cut our losses

    fn approx_size(&self) -> usize {
        0
    }
}
