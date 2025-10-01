use crate::size_hint::SizeHint;
use bytebuffer::ByteBuffer;
use bytes::Bytes;
use std::io;

pub trait SerBytes {
    fn from_buf(buf: &mut ByteBuffer) -> io::Result<Self>
    where
        Self: Sized;

    fn to_buf(&self, buf: &mut ByteBuffer);

    fn from_bytes(bytes: &[u8]) -> io::Result<Self>
    where
        Self: Sized,
    {
        let mut buf = ByteBuffer::from_bytes(bytes);
        Self::from_buf(&mut buf)
    }

    fn to_bb(&self) -> ByteBuffer {
        let mut buf = ByteBuffer::new();
        self.to_buf(&mut buf);
        buf
    }

    fn to_bytes(&self) -> Bytes {
        Bytes::from(self.to_bb().into_vec())
    }

    /// The approximate size of the serialized data, not the amount of memory the data takes up during runtime.
    /// This doesn't need to encapsulate the maximum size of a type. In fact if it does (i.e. with string being isize::max) it will most likely cause oom errors and crash

    fn size_hint() -> SizeHint
    where
        Self: Sized,
    {
        SizeHint::bytes(0)
    }
}
