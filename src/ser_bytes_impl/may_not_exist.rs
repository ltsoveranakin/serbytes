use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// For field values which may not exist. Data in this type must be initialized when serializing
///
/// This should only be used on struct fields that can ensure no other data is stored after in the ByteBuffer
///
/// This panics when serializing to a buffer if Self is [`MayNotExist::DoesNotExist`]
/// For a type that doesn't panic on write, use [`MayNotExistOrElse`]
pub enum MayNotExist<S> {
    Exists(S),
    DoesNotExist,
}

impl<S: SerBytes> SerBytes for MayNotExist<S> {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        if let Ok(s) = S::from_buf(buf) {
            Ok(Self::Exists(s))
        } else {
            Ok(Self::DoesNotExist)
        }
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        match self {
            Self::Exists(s) => {
                s.to_buf(buf);
            }

            Self::DoesNotExist => {
                panic!("Cannot write to buf, value does not exist");
            }
        }
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }

    fn approx_size(&self) -> usize {
        match self {
            Self::Exists(s) => s.approx_size(),

            Self::DoesNotExist => 0,
        }
    }
}

pub trait MayNotExistDataProvider<T> {
    fn get_data() -> T;
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct MayNotExistOrElse<S, F> {
    inner: S,
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
        let data = if let MayNotExist::Exists(data) =
            MayNotExist::from_buf(buf).expect("from_buf method on MayNotExist NEVER returns Err")
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

#[derive(Debug, Default, Eq, PartialEq)]
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
}

impl<S, F> Deref for MayNotExistOrElse<S, F> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S, F> DerefMut for MayNotExistOrElse<S, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> From<T> for MayNotExist<T> {
    fn from(value: T) -> Self {
        Self::Exists(value)
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
