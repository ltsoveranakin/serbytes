mod byte_tag;
pub mod collections;
mod deref_types;
#[cfg(feature = "glam")]
pub mod glam;
mod json_like;
mod may_not_exist;
mod ser_bytes_impl_macro;
mod skip_ser;
mod versioning_wrapper;

pub use byte_tag::*;
pub use json_like::*;
pub use may_not_exist::*;
pub use skip_ser::*;
pub use versioning_wrapper::*;

use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, ReadError, WriteByteBufferOwned};
use crate::ser_trait::{SerBytes, SerBytesStaticSized};

use crate::ser_bytes_impl::ser_bytes_impl_macro::ser_data_impl;
use std::marker::PhantomData;

ser_data_impl!(bool, bool, 1);

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
    S::to_buf(s, buf)
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

impl SerBytesStaticSized for () {}

impl<T> SerBytes for PhantomData<T> {
    fn from_buf(_: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(PhantomData)
    }

    fn to_buf(&self, _: &mut WriteByteBufferOwned) {}
}

impl<T> SerBytesStaticSized for PhantomData<T> {}

impl<S> SerBytes for Option<S>
where
    S: SerBytes,
{
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

impl<'a, S, E> SerBytes for Result<S, E>
where
    S: SerBytes,
    E: From<ReadError<'a>>,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(S::from_buf(buf).map_err(|e| e.into()))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        match &self {
            Ok(s) => {
                s.to_buf(buf);
            }

            Err(_) => {
                panic!("Attempt to write error variant to buffer")
            }
        }
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }

    fn approx_size(&self) -> usize {
        match self {
            Ok(s) => s.approx_size(),
            Err(_) => S::size_hint(),
        }
    }
}

impl<S> SerBytesStaticSized for Option<S> where S: SerBytesStaticSized {}
