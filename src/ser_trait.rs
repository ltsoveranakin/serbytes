use crate::bytebuffer::{
    BBReadResult, ReadByteBufferOwned, ReadByteBufferRefMut, WriteByteBufferOwned,
};
#[cfg(feature = "bytes")]
use bytes::Bytes;

/// [`SerBytes`]
///
/// A type that can be both serialized and deserialized from a binary source.
pub trait SerBytes {
    /// Reads and deserializes the type from the provided [`ReadByteBufferRefMut`]
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized;

    /// Serializes and writes the type to the provided [`WriteByteBufferOwned`]
    fn to_buf(&self, buf: &mut WriteByteBufferOwned);

    fn from_vec(vec: Vec<u8>) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let mut buf = ReadByteBufferOwned::from_vec(vec);

        Self::from_buf(&mut buf.rbb_ref_mut())
    }

    fn from_bytes(bytes: &[u8]) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let mut index = 0;
        let mut bit_index = 0;

        let mut rbb = ReadByteBufferRefMut::from_bytes(bytes, &mut index, &mut bit_index);

        Self::from_buf(&mut rbb)
    }

    fn to_bb(&self) -> WriteByteBufferOwned {
        let mut buf = WriteByteBufferOwned::with_capacity(self.approx_size());
        self.to_buf(&mut buf);
        buf
    }

    #[cfg(feature = "bytes")]
    fn to_bytes_type(&self) -> Bytes {
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

    /// The approximate size of a type at runtime. Statically sized types when serialized (i.e. primitives) should just call Self::size_hint from this function.
    ///
    /// Types that can have varying sizes of serialized data (enums, hashmaps, etc.) should do their best to approximate the size as cheaply as possible

    fn approx_size(&self) -> usize {
        0
    }
}

/// Marker trait that dictates a type will always have the same size when serialized.
///
/// Special care should be taken to ensure this is only implemented on types that will never have a variable serialization size
///
/// Should never be implemented on vectors, maps, enums (unless all enum variants have the exact same size)

pub trait SerBytesStaticSized: SerBytes {}
