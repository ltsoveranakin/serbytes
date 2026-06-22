use crate::bytebuffer::{
    BBReadResult, ReadByteBufferRefMut, ReadByteBufferSlice, WithParent, WriteByteBufferOwned,
};
use crate::ser_bytes_impl::{LengthLike, from_buf};
use crate::ser_trait::{SerBytes, SerBytesStaticSized};
use std::marker::PhantomData;

pub type ResultBlock<T> = SizedBlock<BBReadResult<T>>;

/// A SizedBlock, used to ensure a given piece of data doesn't encroach on the following data in the buffer
///
/// Useful when frequently modifying your data structure especially when combined with [`BBReadResult`], i.e. [`SizedBlock<BBReadResult<SomeTypeThatChanges>>`]

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SizedBlock<S, L = u16> {
    pub inner: S,
    _len: PhantomData<L>,
}

impl<S> SizedBlock<S, u16> {
    pub fn new(block_data: S) -> Self {
        Self {
            inner: block_data,
            _len: PhantomData,
        }
    }
}

impl<S, L> SizedBlock<S, L> {
    pub fn new_with_len_type(block_data: S) -> Self {
        Self {
            inner: block_data,
            _len: PhantomData,
        }
    }
}

impl<S, L> SerBytes for SizedBlock<S, L>
where
    S: SerBytes,
    L: SerBytes + LengthLike + SerBytesStaticSized,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let mut inner_fn = || {
            let len = L::from_buf(buf)?.to_usize();
            let bytes = buf.read_bytes(len)?;

            let mut block_buffer = ReadByteBufferSlice::new(bytes);

            Ok(Self {
                inner: from_buf(&mut block_buffer.rbb_ref_mut())?,
                _len: PhantomData,
            })
        };

        inner_fn().with_parent("SizedBlock")
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        let len_index_ptr = buf.write_with_index_pointer(&L::from_usize(0));

        let data_index_ptr = buf.write_with_index_pointer(&self.inner);

        let data_len = data_index_ptr.len();

        buf.write_at_index_pointer(len_index_ptr, &L::from_usize(data_len));
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        L::size_hint() + S::size_hint()
    }

    fn approx_size(&self) -> usize {
        L::size_hint() + self.inner.approx_size()
    }
}

impl<S, L> Default for SizedBlock<S, L>
where
    S: Default,
{
    fn default() -> Self {
        Self::new_with_len_type(S::default())
    }
}

impl<S, L> From<S> for SizedBlock<S, L> {
    fn from(value: S) -> Self {
        Self::new_with_len_type(value)
    }
}

impl<T> ResultBlock<T> {
    pub fn unwrapped_ref(&self) -> &T {
        self.inner.as_ref().unwrap()
    }

    pub fn unwrapped_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap()
    }
}
