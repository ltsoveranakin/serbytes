use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::bytebuffer;
use crate::bytebuffer::{ReadByteBuffer, WriteByteBuffer};
use crate::prelude::{from_buf, SerBytes};

impl<K, V> SerBytes for HashMap<K, V>
where
    K: SerBytes + Eq + Hash,
    V: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::BBReadResult<Self> {
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

    fn size_hint() -> usize
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
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::BBReadResult<Self> {
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

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        u16::size_hint()
    }
}
