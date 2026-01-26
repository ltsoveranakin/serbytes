use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::prelude::{from_buf, SerBytes};
use crate::ser_trait::SerBytesStaticSized;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

impl<S> SerBytes for Arc<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self::new(from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        S::to_buf(self, buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }
}

impl<S> SerBytesStaticSized for Arc<S> where S: SerBytesStaticSized {}

impl<S> SerBytes for Rc<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        S::to_buf(self, buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }
}

impl<S> SerBytesStaticSized for Rc<S> where S: SerBytesStaticSized {}

impl<S> SerBytes for RefCell<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    /// Panics if the [RefCell] value is being mutable borrowed.

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        S::to_buf(&*self.borrow(), buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }
}

impl<S> SerBytesStaticSized for RefCell<S> where S: SerBytesStaticSized {}

impl<S> SerBytes for Box<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        S::to_buf(self, buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }
}

impl<S> SerBytesStaticSized for Box<S> where S: SerBytesStaticSized {}
