use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::prelude::{SerBytes, from_buf};
use crate::ser_trait::SerBytesStaticSized;
use std::cell::{Cell, RefCell};
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

impl<S> SerBytes for Cell<S>
where
    S: SerBytes + Copy,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(Cell::new(S::from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.get().to_buf(buf);
    }
}

impl<S> SerBytesStaticSized for Cell<S> where S: SerBytesStaticSized + Copy {}
