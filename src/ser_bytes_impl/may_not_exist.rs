use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::marker::PhantomData;

pub trait MayNotExistDataProvider<T> {
    fn get_data() -> T;
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct MayNotExistOrElse<S, F> {
    pub inner: S,
    _callback: PhantomData<F>,
}

impl<S, F> SerBytes for MayNotExistOrElse<S, F>
where
    S: SerBytes,
    F: MayNotExistDataProvider<S>,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let data = if let Ok(data) =
            BBReadResult::from_buf(buf).expect("from_buf method on MayNotExist NEVER returns Err")
        {
            data
        } else {
            F::get_data()
        };

        Ok(Self {
            inner: data,
            _callback: PhantomData,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.inner.to_buf(buf);
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct DefaultDataProvider;

impl<T> MayNotExistDataProvider<T> for DefaultDataProvider
where
    T: Default,
{
    fn get_data() -> T {
        T::default()
    }
}

pub type MayNotExistOrDefault<S> = MayNotExistOrElse<S, DefaultDataProvider>;

impl<S, F> MayNotExistOrElse<S, F> {
    pub fn into_inner(self) -> S {
        self.inner
    }

    pub fn new(s: S) -> Self {
        Self {
            inner: s,
            _callback: PhantomData,
        }
    }
}

impl<T, F> From<T> for MayNotExistOrElse<T, F> {
    fn from(value: T) -> Self {
        Self {
            inner: value,
            _callback: PhantomData,
        }
    }
}
