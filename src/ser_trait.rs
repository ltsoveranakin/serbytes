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

    /// The absolute minimum amount of data that the serialized data will take up in bytes
    /// If it uses less than a byte, should always round up. i.e. 3 bits -> 8 bits (1 byte)
    fn size_hint() -> u16
    where Self:Sized
    {
        0
    }


}
