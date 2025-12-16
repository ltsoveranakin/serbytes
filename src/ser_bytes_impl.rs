use crate::bytebuffer;
use crate::bytebuffer::{ReadByteBuffer, ReadError, WriteByteBuffer};
use crate::ser_bytes_impl_macro::ser_data_impl;
use crate::ser_trait::SerBytes;
use glam::{IVec2, Vec2};
use hashbrown::{HashMap, HashSet};
use std::cell::RefCell;
use std::hash::Hash;
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
pub fn from_buf<S>(buf: &mut ReadByteBuffer) -> bytebuffer::Result<S>
where
    S: SerBytes,
{
    S::from_buf(buf)
}

#[inline]
pub fn to_buf<S>(s: &S, buf: &mut WriteByteBuffer)
where
    S: SerBytes,
{
    s.to_buf(buf);
}

impl SerBytes for bool {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self>
    where
        Self: Sized,
    {
        Ok(buf.read_bit()? == 1)
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        buf.write_bit(*self);
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        1
    }
}

impl SerBytes for String {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        let len = buf.read_u16()?;
        let bytes =
            buf.read_bytes_err_msg(len as usize, format!("bytes for string; {} bytes", len))?;

        String::from_utf8(bytes.to_vec()).map_err(|_| ReadError {
            message: "invalid utf8 for string".into(),
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        buf.write_u16(self.len() as u16);
        buf.write_bytes(self.as_bytes());
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        u16::size_hint()
    }
}

impl<S: SerBytes> SerBytes for Option<S> {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self>
    where
        Self: Sized,
    {
        let is_some = from_buf::<bool>(buf)?;

        Ok(if is_some { Some(from_buf(buf)?) } else { None })
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
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

    fn size_hint() -> u16 {
        bool::size_hint()
    }
}

impl SerBytes for IVec2 {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        Ok(Self {
            x: from_buf(buf)?,
            y: from_buf(buf)?,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        self.x.to_buf(buf);
        self.y.to_buf(buf);
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        u16::size_hint() * 2
    }
}

impl SerBytes for Vec2 {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        Ok(Self {
            x: from_buf(buf)?,
            y: from_buf(buf)?,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        self.x.to_buf(buf);
        self.y.to_buf(buf);
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        f32::size_hint() * 2
    }
}

impl<S: SerBytes> SerBytes for Vec<S> {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        let vec_len = u16::from_buf(buf)? as usize;
        let mut vec = Vec::with_capacity(vec_len);

        for _ in 0..vec_len {
            vec.push(from_buf(buf)?);
        }

        Ok(vec)
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        (self.len() as u16).to_buf(buf);

        for ser_data in self {
            ser_data.to_buf(buf);
        }
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        u16::size_hint()
    }
}

impl<K, V> SerBytes for HashMap<K, V>
where
    K: SerBytes + Eq + Hash,
    V: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        let len = u16::from_buf(buf)? as usize;
        let mut map = Self::with_capacity(len);

        for _ in 0..len {
            let key = from_buf(buf)?;
            let value = from_buf(buf)?;

            map.insert(key, value);
        }

        Ok(map)
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        (self.len() as u16).to_buf(buf);

        for (key, value) in self {
            key.to_buf(buf);
            value.to_buf(buf);
        }
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        u16::size_hint()
    }
}

impl<K> SerBytes for HashSet<K>
where
    K: SerBytes + Eq + Hash,
{
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        let len = u16::from_buf(buf)?;
        let mut set = HashSet::with_capacity(len as usize);

        for _ in 0..len {
            set.insert(from_buf(buf)?);
        }

        Ok(set)
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        (self.len() as u16).to_buf(buf);
        for key in self.iter() {
            key.to_buf(buf);
        }
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        u16::size_hint()
    }
}

impl<S> SerBytes for Arc<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        Ok(Self::new(from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        S::to_buf(self, buf);
    }

    fn size_hint() -> u16
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
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        S::to_buf(self, buf);
    }

    fn size_hint() -> u16
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
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::Result<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    /// Panics if the [RefCell] value is being mutable borrowed.

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        S::to_buf(&*self.borrow(), buf);
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        S::size_hint()
    }
}
