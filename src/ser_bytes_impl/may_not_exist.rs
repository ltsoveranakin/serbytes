use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::any::type_name;
use std::fmt::{Debug, Formatter, Pointer};
use std::marker::PhantomData;

pub type MayNotExistOrDefault<S> = MayNotExistOrElse<S, DefaultDataProvider>;

pub trait MayNotExistDataProvider<T> {
    fn get_data() -> T;
}

pub struct MayNotExistOrElse<S, P> {
    pub inner: S,
    _data_provider: PhantomData<P>,
}

impl<S, P> SerBytes for MayNotExistOrElse<S, P>
where
    S: SerBytes,
    P: MayNotExistDataProvider<S>,
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
            P::get_data()
        };

        Ok(Self {
            inner: data,
            _data_provider: PhantomData,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.inner.to_buf(buf);
    }

    /// Even though the type may not exist, we always write data to the buffer
    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }

    fn approx_size(&self) -> usize {
        self.inner.approx_size()
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

impl<S, P> MayNotExistOrElse<S, P> {
    pub fn into_inner(self) -> S {
        self.inner
    }

    pub fn new(s: S) -> Self {
        Self {
            inner: s,
            _data_provider: PhantomData,
        }
    }
}

impl<T, P> From<T> for MayNotExistOrElse<T, P> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<S, P> Clone for MayNotExistOrElse<S, P>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _data_provider: PhantomData,
        }
    }
}

impl<S, P> Copy for MayNotExistOrElse<S, P> where S: Copy {}

impl<S, P> Default for MayNotExistOrElse<S, P>
where
    S: Default,
{
    fn default() -> Self {
        S::default().into()
    }
}

impl<S, P> Debug for MayNotExistOrElse<S, P>
where
    S: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!(
            "MayNotExistOrElse<{}, {}>",
            type_name::<S>(),
            type_name::<P>()
        ))
        .field("inner", &self.inner)
        .field("_data_provider", &self._data_provider)
        .finish()
    }
}

impl<S, P> PartialEq for MayNotExistOrElse<S, P>
where
    S: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<S, P> Eq for MayNotExistOrElse<S, P> where S: Eq {}
