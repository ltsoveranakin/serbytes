use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, ReadError, WriteByteBufferOwned};
use crate::prelude::SerBytes;

impl<'a, S, E> SerBytes for Result<S, E>
where
    S: SerBytes,
    E: From<ReadError<'a>>,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        Ok(S::from_buf(buf).map_err(|e| e.into()))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        match &self {
            Ok(s) => {
                s.to_buf(buf);
            }

            Err(_) => {
                panic!("Attempt to write error variant to buffer")
            }
        }
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        S::size_hint()
    }

    fn approx_size(&self) -> usize {
        match self {
            Ok(s) => s.approx_size(),
            Err(_) => S::size_hint(),
        }
    }
}
