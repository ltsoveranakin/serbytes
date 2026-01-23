use crate::bytebuffer::{ReadByteBuffer, WriteByteBuffer};
use crate::ser_trait::SerBytes;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct WriteSerByteTag<S: SerBytes, V: SerBytes> {
    wbb: WriteByteBuffer,
    _tag_key: PhantomData<S>,
    _tag_value: PhantomData<V>,
}

impl<K: SerBytes, V: SerBytes> WriteSerByteTag<K, V> {
    pub fn new() -> Self {
        Self {
            wbb: WriteByteBuffer::new(),
            _tag_key: PhantomData,
            _tag_value: PhantomData,
        }
    }

    pub fn write_tag(&mut self, tag_key: K, tag_value: V) {
        tag_key.to_buf(&mut self.wbb);
        tag_value.to_buf(&mut self.wbb);
    }

    pub fn get_buf(self) -> WriteByteBuffer {
        self.wbb
    }
}

pub struct ReadSerByteTag<K: SerBytes, V: SerBytes> {
    tags: HashMap<K, V>,
}

impl<K: SerBytes, V: SerBytes> ReadSerByteTag<K, V> {
    pub fn from_buf(buf: ReadByteBuffer) -> Self {
        let mut tags = HashMap::new();

        Self { tags }
    }

    pub fn write_tag(&mut self, tag_key: K, tag_value: V) {
        tag_key.to_buf(&mut self.wbb);
        tag_value.to_buf(&mut self.wbb);
    }

    pub fn get_buf(self) -> WriteByteBuffer {
        self.wbb
    }
}
