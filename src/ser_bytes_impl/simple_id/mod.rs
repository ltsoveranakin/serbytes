use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::ser_trait::SerBytes;
use simple_id::prelude::Id;

impl SerBytes for Id {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let maybe_header = u8::from_buf(buf)?;

        let header;
        let increment;

        if Id::is_header(maybe_header) {
            header = maybe_header;
            increment = u8::from_buf(buf)?;
        } else {
            header = 0b00000000;
            increment = maybe_header;
        }

        let time = u16::from_buf(buf)?;

        let data = buf
            .read_bytes(12)?
            .try_into()
            .expect("Just read 12 bytes from the buffer, can't fail");

        Ok(Self {
            header,
            increment,
            time,
            data,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        if self.has_header() {
            self.header.to_buf(buf);
        }

        self.increment.to_buf(buf);
        self.time.to_buf(buf);
        buf.write_bytes(&self.data);
    }
}

// not static sized
