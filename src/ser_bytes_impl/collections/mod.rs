use crate::bytebuffer;
use crate::bytebuffer::{ReadByteBuffer, ReadError, WriteByteBuffer};
use crate::prelude::{from_buf, SerBytes};
pub mod hashmap;

impl<S: SerBytes> SerBytes for Vec<S> {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::BBReadResult<Self> {
        let vec_len = u16::from_buf(buf)? as usize;
        let mut vec = Vec::with_capacity(vec_len);

        for _ in 0..vec_len {
            vec.push(from_buf(buf)?);
        }

        Ok(vec)
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        (self.len() as u16).to_buf(buf);

        for ser_data in self {
            ser_data.to_buf(buf);
        }
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        u16::size_hint()
    }

    fn approx_size(&self) -> usize {
        u16::size_hint() + (S::size_hint() * self.len())
    }
}

impl SerBytes for String {
    fn from_buf(buf: &mut ReadByteBuffer) -> bytebuffer::BBReadResult<Self> {
        let len = u16::from_buf(buf)? as usize;
        let bytes = buf.read_bytes_with_err_msg(len, format!("bytes for string; {} bytes", len))?;

        String::from_utf8(bytes.to_vec())
            .map_err(|_| ReadError::new("invalid utf8 for string".into()))
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        buf.write_u16(self.len() as u16);
        buf.write_bytes(self.as_bytes());
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        u16::size_hint()
    }

    fn approx_size(&self) -> usize {
        u16::size_hint() + self.len()
    }
}
