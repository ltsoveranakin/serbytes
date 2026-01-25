use crate::bytebuffer::{BBReadResult, IndexPointer, ReadByteBuffer, WriteByteBuffer};
use crate::ser_trait::SerBytes;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct WriteSerByteTag<S: SerBytes, V: SerBytes> {
    wbb: WriteByteBuffer,
    len: usize,
    len_index_ptr: IndexPointer<u16>,
    _tag_key: PhantomData<S>,
    _tag_value: PhantomData<V>,
}

impl<K: SerBytes, V: SerBytes> WriteSerByteTag<K, V> {
    pub fn new() -> Self {
        let mut wbb = WriteByteBuffer::with_capacity(u16::size_hint());

        let len_index_ptr = wbb.write_u16(0);

        Self {
            wbb,
            len: 0,
            len_index_ptr,
            _tag_key: PhantomData,
            _tag_value: PhantomData,
        }
    }

    pub fn write_tag(&mut self, tag_key: K, tag_value: V) {
        self.wbb
            .reserve(tag_key.approx_size() + tag_value.approx_size());

        tag_key.to_buf(&mut self.wbb);
        tag_value.to_buf(&mut self.wbb);

        self.len += 1;

        self.wbb
            .write_at_index_pointer(&self.len_index_ptr, self.len as u16);
    }

    pub fn get_buf(self) -> WriteByteBuffer {
        self.wbb
    }
}

pub struct ReadSerByteTag<K: SerBytes + Eq + Hash, V: SerBytes> {
    tags: HashMap<K, V>,
}

impl<K: SerBytes + Eq + Hash, V: SerBytes> ReadSerByteTag<K, V> {
    pub fn from_buf(mut rbb: ReadByteBuffer) -> BBReadResult<Self> {
        let len = u16::from_buf(&mut rbb)?;

        let mut tags = HashMap::with_capacity(len as usize);

        for _ in 0..len {
            let key = K::from_buf(&mut rbb)?;
            let value = V::from_buf(&mut rbb)?;
            tags.insert(key, value);
        }

        Ok(Self { tags })
    }

    pub fn get_tag(&mut self, tag_key: &K) -> Option<&V> {
        self.tags.get(tag_key)
    }
}
