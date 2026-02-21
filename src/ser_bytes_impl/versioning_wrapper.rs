use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub trait VersioningDataProvider<V, I, O> {
    fn get_versioned_data(version: V, data_input: I) -> O;
}

pub trait CurrentVersion {
    fn get_current_version() -> Self;
}

pub struct UnchangedVersioningDataProvider<V, O> {
    _version: PhantomData<V>,
    _outgoing: PhantomData<O>,
}

impl<V, O> VersioningDataProvider<V, O, O> for UnchangedVersioningDataProvider<V, O> {
    fn get_versioned_data(_: V, data_input: O) -> O {
        data_input
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct VersioningWrapper<S, V, I, F> {
    inner: S,
    _version: PhantomData<V>,
    _incoming: PhantomData<I>,
    _callback: PhantomData<F>,
}

impl<V, I, O, F> SerBytes for VersioningWrapper<O, V, I, F>
where
    V: SerBytes + CurrentVersion,
    I: SerBytes,
    O: SerBytes,
    F: VersioningDataProvider<V, I, O>,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let version = V::from_buf(buf)?;
        let incoming_data = I::from_buf(buf)?;

        let outgoing = F::get_versioned_data(version, incoming_data);

        Ok(Self {
            inner: outgoing,
            _version: PhantomData,
            _incoming: PhantomData,
            _callback: PhantomData,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        V::get_current_version().to_buf(buf);

        self.inner.to_buf(buf);
    }
}

impl<S, V, I, F> Deref for VersioningWrapper<S, V, I, F> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S, V, I, F> DerefMut for VersioningWrapper<S, V, I, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<S, V, I, F> From<S> for VersioningWrapper<S, V, I, F> {
    fn from(value: S) -> Self {
        Self {
            inner: value,
            _version: PhantomData,
            _incoming: PhantomData,
            _callback: PhantomData,
        }
    }
}

impl<S, V, I, F> VersioningWrapper<S, V, I, F> {
    pub fn into_inner(self) -> S {
        self.inner
    }
}
