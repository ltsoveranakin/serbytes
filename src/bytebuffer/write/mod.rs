mod bb_ref;
mod owned;
mod write_macro;

use std::marker::PhantomData;

use crate::ser_trait::SerBytes;
pub use owned::*;

#[derive(Copy, Clone, Debug)]
pub struct IndexPointer<S>
where
    S: SerBytes,
{
    index: usize,
    len: usize,
    _s: PhantomData<S>,
}

impl<S> IndexPointer<S>
where
    S: SerBytes,
{
    /// 99% of cases you shouldn't be constructing this and should obtain it through the [`WriteByteBufferOwned::write_with_index_pointer`] method
    pub fn new(index: usize, len: usize) -> Self {
        Self {
            index,
            len,
            _s: PhantomData,
        }
    }
}
