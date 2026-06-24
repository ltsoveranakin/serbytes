use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::marker::PhantomData;

pub struct Mapped<S, M> {
    pub inner: S,
    _mapped_provider: PhantomData<M>,
}

pub trait MappedDataProvider<S> {
    fn value_from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<S>;

    fn value_to_buf(value: &S, buf: &mut WriteByteBufferOwned);
}

impl<S, M> SerBytes for Mapped<S, M>
where
    M: MappedDataProvider<S>,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            inner: M::value_from_buf(buf)?,
            _mapped_provider: PhantomData,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        M::value_to_buf(&self.inner, buf)
    }
}
