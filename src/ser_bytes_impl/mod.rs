pub mod collections;

use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_bytes_impl_macro::ser_data_impl;
use crate::ser_trait::SerBytes;
use glam::{IVec2, Vec2};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

ser_data_impl!(u8, u8, 1);
ser_data_impl!(u16, u16, 2);
ser_data_impl!(u32, u32, 4);
ser_data_impl!(u64, u64, 8);
ser_data_impl!(u128, u128, 16);

ser_data_impl!(i8, i8, 1);
ser_data_impl!(i16, i16, 2);
ser_data_impl!(i32, i32, 4);
ser_data_impl!(i64, i64, 8);
ser_data_impl!(i128, i128, 16);

ser_data_impl!(f32, f32, 4);
ser_data_impl!(f64, f64, 8);

#[inline]
pub fn from_buf<S>(buf: &mut ReadByteBufferRefMut) -> BBReadResult<S>
where
    S: SerBytes,
{
    S::from_buf(buf)
}

#[inline]
pub fn to_buf<S>(s: &S, buf: &mut WriteByteBufferOwned)
where
    S: SerBytes,
{
    s.to_buf(buf);
}

impl SerBytes for () {
    fn from_buf(_: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(())
    }

    fn to_buf(&self, _: &mut WriteByteBufferOwned) {}
}

impl<T> SerBytes for PhantomData<T> {
    fn from_buf(_: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(PhantomData)
    }

    fn to_buf(&self, _: &mut WriteByteBufferOwned) {}
}

impl SerBytes for bool {
    #[inline]
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        buf.read_bool()
    }

    #[inline]
    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        buf.write_bool(*self);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        1
    }
}

impl<S: SerBytes> SerBytes for Option<S> {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let is_some = from_buf::<bool>(buf)?;

        Ok(if is_some { Some(from_buf(buf)?) } else { None })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        match self {
            Some(s) => {
                true.to_buf(buf);
                s.to_buf(buf);
            }

            None => {
                false.to_buf(buf);
            }
        }
    }

    fn size_hint() -> usize {
        bool::size_hint()
    }
}

impl SerBytes for IVec2 {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self {
            x: from_buf(buf)?,
            y: from_buf(buf)?,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.x.to_buf(buf);
        self.y.to_buf(buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        u16::size_hint() * 2
    }
}

impl SerBytes for Vec2 {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self {
            x: from_buf(buf)?,
            y: from_buf(buf)?,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.x.to_buf(buf);
        self.y.to_buf(buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        f32::size_hint() * 2
    }
}

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
