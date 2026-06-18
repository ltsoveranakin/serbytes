use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};

pub trait SerbytesMapped<T> {
    fn from_buf_mapped<F>(buf: &mut ReadByteBufferRefMut, reader: F) -> BBReadResult<Self>
    where
        Self: Sized,
        F: FnOnce(&mut ReadByteBufferRefMut) -> BBReadResult<T>;

    fn to_buf_mapped<F>(&self, buf: &mut WriteByteBufferOwned, writer: F)
    where
        F: FnOnce(&mut WriteByteBufferOwned, &T);
}
