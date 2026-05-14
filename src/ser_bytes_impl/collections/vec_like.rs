use crate::bytebuffer;
use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WithParent, WriteByteBufferOwned};
use crate::ser_bytes_impl::{LengthLike, from_buf};
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
    /// NOTE: This is incredibly inefficient for a [`Vec`] of `u8` or `i8`, instead use [`U8Vec`] or [`I8Vec`] respectively
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> bytebuffer::BBReadResult<Self> {
        let mut inner = || {
            let vec_len = u16::from_buf(buf)? as usize;
            let mut vec = Vec::with_capacity(vec_len);

            for _ in 0..vec_len {
                vec.push(from_buf(buf)?);
            }

            Ok(vec)
        };

        inner().with_parent("Vec")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        (self.len() as u16).to_buf(buf);

        for ser_data in self {
            ser_data.to_buf(buf);
        }
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
    __len: PhantomData<L>,
}

impl<L> SerBytes for U8Vec<L>
where
    L: SerBytes + LengthLike,
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
                __len: PhantomData,
            })
        };

        inner().with_parent("U8Vec")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        let len = L::from_usize(self.vec.len());

        len.to_buf(buf);
        buf.write_bytes(&self.vec);
    }
}

impl<L> U8Vec<L> {
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self {
            vec,
            __len: PhantomData,
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
