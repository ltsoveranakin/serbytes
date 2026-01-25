use crate::bytebuffer;
use crate::bytebuffer::{ReadByteBuffer, WriteByteBuffer};
use bytes::Bytes;

pub trait SerBytes {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::BBReadResult<Self>
    where
        Self: Sized;

    fn to_buf(&self, buf: &mut WriteByteBuffer);

    fn from_vec(vec: Vec<u8>) -> bytebuffer::BBReadResult<Self>
    where
        Self: Sized,
    {
        let mut buf = ReadByteBuffer::from_vec(vec);
        Self::from_buf(&mut buf)
    }

    fn from_bytes(bytes: &[u8]) -> bytebuffer::BBReadResult<Self>
    where
        Self: Sized,
    {
        Self::from_vec(bytes.to_vec())
    }

    fn to_bb(&self) -> WriteByteBuffer {
        let mut buf = WriteByteBuffer::with_capacity(self.approx_size());
        self.to_buf(&mut buf);
        buf
    }

    fn to_bytes(&self) -> Bytes {
        Bytes::from(self.to_bb().into_vec())
    }

    /// The absolute minimum amount of data that the serialized data will take up in bytes
    /// If it uses less than a byte, should always round up. i.e. 3 bits -> 8 bits (1 byte)
    fn size_hint() -> usize
    where
        Self: Sized,
    {
        0
    }

    /// The approximate size of a type at runtime. Statically sized types when serialized (ie. primitives) should just call Self::size_hint from this function.
    ///
    /// Types that can have varying sizes of serialized data (enums, hashmaps, etc.) should do their best to approximate the size as cheaply as possible

    fn approx_size(&self) -> usize {
        0
    }
}
