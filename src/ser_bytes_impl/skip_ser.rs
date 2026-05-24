use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};

use crate::ser_trait::SerBytes;
use std::marker::PhantomData;

pub trait FallbackDataProvider<S> {
    fn get_data() -> S;
}

pub struct SkipSerialization<S, F> {
    inner: S,
    _data_provider: PhantomData<F>,
}

impl<S, F> SerBytes for SkipSerialization<S, F>
where
    F: FallbackDataProvider<S>,
{
    fn from_buf(_: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            inner: F::get_data(),
            _data_provider: PhantomData,
        })
    }

    fn to_buf(&self, _: &mut WriteByteBufferOwned) {}
}

impl<S, F> SkipSerialization<S, F> {
    pub fn into_inner(self) -> S {
        self.inner
    }
}
