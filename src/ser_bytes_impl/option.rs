use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::mapped::SerbytesMapped;
use crate::ser_trait::SerBytes;

impl<S> SerBytes for Option<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Self::from_buf_mapped(buf, |buf| S::from_buf(buf))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.to_buf_mapped(buf, |buf, value| {
            value.to_buf(buf);
        })
    }

    fn size_hint() -> usize {
        bool::size_hint()
    }

    fn approx_size(&self) -> usize {
        bool::size_hint()
    }
}

impl<T> SerbytesMapped<T> for Option<T> {
    fn from_buf_mapped<F>(buf: &mut ReadByteBufferRefMut, reader: F) -> BBReadResult<Self>
    where
        Self: Sized,
        F: FnOnce(&mut ReadByteBufferRefMut) -> BBReadResult<T>,
    {
        if buf.read_bool()? {
            Ok(Some(reader(buf)?))
        } else {
            Ok(None)
        }
    }

    fn to_buf_mapped<F>(&self, buf: &mut WriteByteBufferOwned, writer: F)
    where
        F: FnOnce(&mut WriteByteBufferOwned, &T),
    {
        if let Some(value) = self {
            true.to_buf(buf);

            writer(buf, value);
        } else {
            false.to_buf(buf);
        }
    }
}
