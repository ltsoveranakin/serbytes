use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::prelude::{SerBytes, from_buf};

impl<S> SerBytes for Option<S>
where
    S: SerBytes,
{
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let is_some = from_buf::<bool>(buf)?;

        Ok(if is_some { Some(from_buf(buf)?) } else { None })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        match self {
            Some(s) => {
                true.to_buf(buf);
                s.to_buf(buf);
            }

            None => {
                false.to_buf(buf);
            }
        }
    }

    fn size_hint() -> usize {
        bool::size_hint()
    }
}
