use crate::prelude::SizeHint;
use crate::ser_bytes_impl_macro::ser_data_impl;
use crate::ser_trait::SerBytes;
use bytebuffer::ByteBuffer;
use glam::{IVec2, Vec2};
use hashbrown::{HashMap, HashSet};
use std::cell::RefCell;
use std::hash::Hash;
use std::io;
use std::rc::Rc;
use std::sync::Arc;

ser_data_impl!(u8, u8);
ser_data_impl!(u16, u16);
ser_data_impl!(u32, u32);
ser_data_impl!(u64, u64);
ser_data_impl!(u128, u128);

ser_data_impl!(i8, i8);
ser_data_impl!(i16, i16);
ser_data_impl!(i32, i32);
ser_data_impl!(i64, i64);
ser_data_impl!(i128, i128);

ser_data_impl!(f32, f32);
ser_data_impl!(f64, f64);

#[inline]
pub fn from_buf<S>(buf: &mut ByteBuffer) -> io::Result<S>
where
    S: SerBytes,
{
    S::from_buf(buf)
}

#[inline]
pub fn to_buf<S>(s: &S, buf: &mut ByteBuffer)
where
    S: SerBytes,
{
    s.to_buf(buf);
}

impl SerBytes for bool {
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self>
    where
        Self: Sized,
    {
        buf.read_bit()
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
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
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        buf.read_string()
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
        buf.write_string(self)
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        u32::size_hint()
    }
}

impl<S: SerBytes> SerBytes for Option<S> {
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self>
    where
        Self: Sized,
    {
        let is_some = from_buf::<bool>(buf)?;

        Ok(if is_some { Some(from_buf(buf)?) } else { None })
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
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
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        Ok(Self {
            x: i32::from_buf(buf)?,
            y: i32::from_buf(buf)?,
        })
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
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
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        Ok(Self {
            x: f32::from_buf(buf)?,
            y: f32::from_buf(buf)?,
        })
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
        self.x.to_buf(buf);
        self.y.to_buf(buf);
    }

    fn size_hint() -> u16
    where
        Self: Sized,
    {
        f32::size_hint()* 2
    }
}

impl<S: SerBytes> SerBytes for Vec<S> {
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        let vec_len = u16::from_buf(buf)? as usize;
        let mut vec = Vec::with_capacity(vec_len);

        for _ in 0..vec_len {
            vec.push(S::from_buf(buf)?);
        }

        Ok(vec)
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
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
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        let len = u16::from_buf(buf)? as usize;
        let mut map = Self::with_capacity(len);

        for _ in 0..len {
            let key = K::from_buf(buf)?;
            let value = V::from_buf(buf)?;

            map.insert(key, value);
        }

        Ok(map)
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
        (self.len() as u16).to_buf(buf);

        for (key, value) in self {
            key.to_buf(buf);
            value.to_buf(buf);
        }
    }
}

impl<K> SerBytes for HashSet<K>
where
    K: SerBytes + Eq + Hash,
{
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        let len = u16::from_buf(buf)?;
        let mut set = HashSet::with_capacity(len as usize);

        for _ in 0..len {
            set.insert(from_buf(buf)?);
        }

        Ok(set)
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
        (self.len() as u16).to_buf(buf);
        for key in self.iter() {
            key.to_buf(buf);
        }
    }
}

impl<S> SerBytes for Arc<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
        S::to_buf(self, buf);
    }
}

impl<S> SerBytes for Rc<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    fn to_buf(&self, buf: &mut ByteBuffer) {
        S::to_buf(self, buf);
    }
}

impl<S> SerBytes for RefCell<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self> {
        Ok(Self::new(S::from_buf(buf)?))
    }

    /// Panics if the [RefCell] value is being mutable borrowed.

    fn to_buf(&self, buf: &mut ByteBuffer) {
        S::to_buf(&*self.borrow(), buf);
    }
}
