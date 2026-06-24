use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WithParent, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::marker::PhantomData;

pub struct Mapped<S, M> {
    pub inner: S,
    _mapped_provider: PhantomData<M>,
}

impl<S, M> Mapped<S, M> {
    pub fn new(value: S) -> Self {
        Self {
            inner: value,
            _mapped_provider: PhantomData,
        }
    }
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
        let mut inner_fn = || {
            Ok(Self {
                inner: M::value_from_buf(buf)?,
                _mapped_provider: PhantomData,
            })
        };

        inner_fn().with_parent("Mapped")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        M::value_to_buf(&self.inner, buf)
    }
}

impl<S, M> Default for Mapped<S, M>
where
    S: Default,
{
    fn default() -> Self {
        Self::new(S::default())
    }
}

impl<S, M> From<S> for Mapped<S, M> {
    fn from(value: S) -> Self {
        Self::new(value)
    }
}
