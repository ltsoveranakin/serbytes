use crate::bytebuffer::{
    BBReadResult, ReadByteBufferOwned, ReadByteBufferRefMut, WithParent, WriteByteBufferOwned,
};
use crate::ser_bytes_impl::{LengthLike, U8Vec, from_buf};
use crate::ser_trait::{SerBytes, SerBytesStaticSized};
use std::marker::PhantomData;

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
            let block_buffer_vec = U8Vec::<L>::from_buf(buf)?;

            let mut block_buffer = ReadByteBufferOwned::from_vec(block_buffer_vec);

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

    fn approx_size(&self) -> usize {
        L::size_hint() + self.inner.approx_size()
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        L::size_hint() + S::size_hint()
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
