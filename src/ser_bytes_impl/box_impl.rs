use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::prelude::{SerBytes, SerBytesStaticSized};

impl<S> SerBytes for Box<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        S::to_buf(self, buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }

    fn approx_size(&self) -> usize {
        S::approx_size(&self)
    }
}

impl<S> SerBytesStaticSized for Box<S> where S: SerBytesStaticSized {}
