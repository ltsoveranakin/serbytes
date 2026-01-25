mod owned;
use std::marker::PhantomData;

pub use owned::*;

pub struct IndexPointer<S> {
    index: usize,
    len: usize,
    _s: PhantomData<S>,
}

impl<S> IndexPointer<S> {
    fn new(index: usize, len: usize) -> Self {
        Self {
            index,
            len,
            _s: PhantomData,
        }
    }
}
