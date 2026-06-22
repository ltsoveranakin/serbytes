use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WithParent, WriteByteBufferOwned};
use crate::ser_bytes_impl::{LengthLike, slice_to_buf_u16, u8_slice_to_buf, vec_from_buf_u16};
use crate::ser_trait::SerBytes;
use std::marker::PhantomData;

impl<S> SerBytes for Vec<S>
where
    S: SerBytes,
{
    /// Reads bytes from a buffer into a [`Vec<S>`]
    ///
    /// Uses a `u16` to determine the amount of bytes to read
    ///
    /// NOTE: This is incredibly inefficient for a [`Vec`] of `u8` or `i8`, instead use [`U8Vec`]
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        let mut inner = || vec_from_buf_u16(buf);

        inner().with_parent("Vec")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        slice_to_buf_u16(buf, self);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        u16::size_hint()
    }

    fn approx_size(&self) -> usize {
        u16::size_hint() + (S::size_hint() * self.len())
    }
}

#[derive(Clone, Debug)]
pub struct U8Vec<L = u16> {
    pub vec: Vec<u8>,
    _len: PhantomData<L>,
}

impl<L> SerBytes for U8Vec<L>
where
    L: LengthLike,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let mut inner = || {
            let len = L::from_buf(buf)?.to_usize();
            let bytes = buf.read_bytes(len)?;

            Ok(Self {
                vec: bytes.to_vec(),
                _len: PhantomData,
            })
        };

        inner().with_parent("U8Vec")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        u8_slice_to_buf::<L>(buf, &self.vec);
    }
}

impl<L> U8Vec<L> {
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            vec,
            _len: PhantomData,
        }
    }
}

impl<L> From<Vec<u8>> for U8Vec<L> {
    fn from(value: Vec<u8>) -> Self {
        Self::from_vec(value)
    }
}

impl<L> From<U8Vec<L>> for Vec<u8> {
    fn from(value: U8Vec<L>) -> Self {
        value.vec
    }
}
