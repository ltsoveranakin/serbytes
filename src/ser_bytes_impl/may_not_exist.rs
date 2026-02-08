use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use std::ops::{Deref, DerefMut};

/// For field values which may not exist. Data in this type must be initialized when serializing
///
/// This should only be used on struct fields that can ensure no other data is stored after in the ByteBuffer
///
/// This panics when serializing to a buffer if Self is [`MayNotExist::DoesNotExist`]
/// For a type that doesn't panic on write, use [`MayNotExistDefault`]
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

#[derive(Debug)]
pub struct MayNotExistDefault<S>(S);

impl<S: SerBytes + Default> SerBytes for MayNotExistDefault<S> {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let data = if let MayNotExist::Exists(data) =
            MayNotExist::from_buf(buf).expect("from_buf method on MayNotExist NEVER returns Err")
        {
            data
        } else {
            S::default()
        };

        Ok(Self(data))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.0.to_buf(buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }

    fn approx_size(&self) -> usize {
        self.0.approx_size()
    }
}

impl<S> MayNotExistDefault<S> {
    pub fn into_inner(self) -> S {
        self.0
    }
}

impl<S: Default> Default for MayNotExistDefault<S> {
    fn default() -> Self {
        Self(S::default())
    }
}

impl<S: Clone> Clone for MayNotExistDefault<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S: Copy> Copy for MayNotExistDefault<S> {}

impl<S: PartialEq> PartialEq for MayNotExistDefault<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(other)
    }
}

impl<S: Eq> Eq for MayNotExistDefault<S> {}

impl<S> Deref for MayNotExistDefault<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for MayNotExistDefault<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for MayNotExist<T> {
    fn from(value: T) -> Self {
        Self::Exists(value)
    }
}

impl<T> From<T> for MayNotExistDefault<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
