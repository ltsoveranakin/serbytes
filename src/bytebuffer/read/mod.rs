use crate::bytebuffer::index_pointer::write::IndexPointerWrite;
use crate::ser_bytes_impl::from_buf;
use crate::ser_trait::SerBytes;
use bytebuffer::prelude::{
    BBReadResult, ReadByteBufferRefMut, ReadError, SpecificError, WriteByteBufferOwned,
};

impl<'s> SerBytes for ReadError<'s> {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let specific_error = from_buf(buf)?;
        let of = String::from_buf(buf)?;
        let child = from_buf(buf)?;

        Ok(Self::new(specific_error, of, child))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.specific_error.to_buf(buf);
        self.of.to_buf(buf);
        self.child.to_buf(buf);
    }
}

impl<'s> SerBytes for SpecificError<'s> {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let ordinal = buf.read_u8()?;

        let s = match ordinal {
            0 => Self::U8,
            1 => Self::Bytes {
                remaining_bytes: from_buf(buf)?,
                got: from_buf(buf)?,
            },
            2 => Self::SingleBit,
            3 => Self::RemainingBits,
            4 => Self::EnumOrdinalOutOfBounds {
                max_bound: from_buf(buf)?,
                got: from_buf(buf)?,
            },
            5 => Self::Other(from_buf(buf)?),
            _ => {
                return Err(ReadError::new(
                    SpecificError::EnumOrdinalOutOfBounds {
                        max_bound: 5,
                        got: ordinal,
                    },
                    "SpecificError",
                    None,
                ));
            }
        };

        Ok(s)
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        let ord_ip = buf.write_with_index_pointer(&0u8);

        let ord = match self {
            Self::U8 => 0,
            Self::Bytes {
                remaining_bytes,
                got,
            } => {
                remaining_bytes.to_buf(buf);
                got.to_buf(buf);

                1
            }
            Self::SingleBit => 2,
            Self::RemainingBits => 3,
            Self::EnumOrdinalOutOfBounds { got, max_bound } => {
                max_bound.to_buf(buf);
                got.to_buf(buf);

                4
            }
            Self::Other(other_str) => {
                other_str.to_buf(buf);

                5
            }
        };

        buf.write_at_index_pointer(ord_ip, &ord);
    }
}
