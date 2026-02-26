use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_bytes_impl::FallbackDataProvider;
use crate::ser_trait::SerBytes;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct SkipSerialization<S, F> {
    data: S,
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
            data: F::get_data(),
            _data_provider: PhantomData,
        })
    }

    fn to_buf(&self, _: &mut WriteByteBufferOwned) {}
}

impl<S, F> Deref for SkipSerialization<S, F> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<S, F> DerefMut for SkipSerialization<S, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<S, F> SkipSerialization<S, F> {
    pub fn into_inner(self) -> S {
        self.data
    }
}
