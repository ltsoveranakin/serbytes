pub mod box_impl;
mod byte_tag;
pub mod collections;
pub mod cow;
mod deref_types;
mod duration;
#[cfg(feature = "glam")]
pub mod glam;
mod json_like;
mod may_not_exist;
pub mod option;
pub mod result;
mod ser_bytes_impl_macro;
mod sized_block;
mod skip_ser;
mod versioning_wrapper;

pub use byte_tag::*;
pub use collections::*;
pub use json_like::*;
pub use may_not_exist::*;
pub use sized_block::*;
pub use skip_ser::*;
use std::cmp::Ordering;
pub use versioning_wrapper::*;

use crate::bytebuffer::{
    BBReadResult, ReadByteBufferRefMut, ReadError, SpecificError, WithParent, WriteByteBufferOwned,
};
use crate::ser_trait::{SerBytes, SerBytesStaticSized};

use crate::ser_bytes_impl::ser_bytes_impl_macro::{ser_data_impl, ser_data_impl_u};
use std::marker::PhantomData;

ser_data_impl!(bool, bool, 1);

ser_data_impl_u!(u8, u8, 1);
ser_data_impl_u!(u16, u16, 2);
ser_data_impl_u!(u32, u32, 4);
ser_data_impl_u!(u64, u64, 8);
ser_data_impl_u!(u128, u128, 16);

ser_data_impl!(i8, i8, 1);
ser_data_impl!(i16, i16, 2);
ser_data_impl!(i32, i32, 4);
ser_data_impl!(i64, i64, 8);
ser_data_impl!(i128, i128, 16);

ser_data_impl!(f32, f32, 4);
ser_data_impl!(f64, f64, 8);

pub trait LengthLike: SerBytes {
    fn from_usize(us: usize) -> Self;

    fn to_usize(self) -> usize;
}

#[inline(always)]
pub fn from_buf<S>(buf: &mut ReadByteBufferRefMut) -> BBReadResult<S>
where
    S: SerBytes,
{
    S::from_buf(buf)
}

#[inline(always)]
pub fn to_buf<S>(s: &S, buf: &mut WriteByteBufferOwned)
where
    S: SerBytes,
{
    S::to_buf(s, buf)
}

pub fn slice_to_buf<S, L>(buf: &mut WriteByteBufferOwned, slice: &[S])
where
    S: SerBytes,
    L: LengthLike,
{
    buf.reserve(S::size_hint() * slice.len());

    L::from_usize(slice.len()).to_buf(buf);

    for s in slice {
        s.to_buf(buf);
    }
}

#[inline(always)]
pub fn slice_to_buf_u16<S>(buf: &mut WriteByteBufferOwned, slice: &[S])
where
    S: SerBytes,
{
    slice_to_buf::<S, u16>(buf, slice)
}

pub fn vec_from_buf<S, L>(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Vec<S>>
where
    S: SerBytes,
    L: LengthLike,
{
    let len = L::from_buf(buf)?.to_usize();

    let mut v = Vec::with_capacity(len);

    for _ in 0..len {
        v.push(S::from_buf(buf)?);
    }

    Ok(v)
}

#[inline(always)]
pub fn vec_from_buf_u16<S>(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Vec<S>>
where
    S: SerBytes,
{
    vec_from_buf::<S, u16>(buf)
}

pub fn into_slice_from_buf<S, L, O>(buf: &mut ReadByteBufferRefMut) -> BBReadResult<O>
where
    S: SerBytes,
    L: LengthLike,
    O: From<Vec<S>>,
{
    let v = vec_from_buf::<S, L>(buf)?;

    Ok(v.into())
}

pub fn into_slice_from_buf_u16<S, O>(buf: &mut ReadByteBufferRefMut) -> BBReadResult<O>
where
    S: SerBytes,
    O: From<Vec<S>>,
{
    let v = vec_from_buf::<S, u16>(buf)?;

    Ok(v.into())
}

pub fn u8_slice_to_buf<L>(buf: &mut WriteByteBufferOwned, slice: &[u8])
where
    L: LengthLike,
{
    L::from_usize(slice.len()).to_buf(buf);

    buf.write_bytes(slice);
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

impl SerBytes for Ordering {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let ord_int = u8::from_buf(buf).with_parent("Ordering")?;

        let ord = match ord_int {
            0 => Self::Less,

            1 => Self::Equal,

            2 => Self::Greater,

            _ => {
                return Err(ReadError::new(
                    SpecificError::Other("Enum index out of bounds".into()),
                    "Ordering",
                    None,
                ));
            }
        };

        Ok(ord)
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        let ord_int = match self {
            Self::Less => 0,
            Self::Equal => 1,
            Self::Greater => 2,
        };

        u8::to_buf(&ord_int, buf);
    }
}
