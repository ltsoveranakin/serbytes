mod bb_ref;
mod owned;
mod write_macro;

use std::marker::PhantomData;

use crate::ser_trait::SerBytes;
pub use owned::*;

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
    fn new(index: usize, len: usize) -> Self {
        Self {
            index,
            len,
            _s: PhantomData,
        }
    }
}
