use crate::bytebuffer::{ReadByteBuffer, WriteByteBuffer};
use crate::prelude::{BBReadResult, SerBytes};
use crate::ser_bytes_impl::from_buf;

/// WIP
struct Size(usize);

impl SerBytes for Size {
    fn from_buf(buf: &mut ReadByteBuffer) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        buf.flush_bits();
        let is_larger_than_7_bits = from_buf::<bool>(buf)?;

        let size_usize = if is_larger_than_7_bits {
            let bytes_needed = buf.read_remaining_bits()?;
            let bytes = buf.read_bytes(bytes_needed as usize)?;
            let mut u128_bytes = [0; 16];

            u128_bytes[(16 - bytes.len())..].copy_from_slice(&bytes);

            let size_u128 = u128::from_be_bytes(u128_bytes);

            size_u128 as usize
        } else {
            buf.read_remaining_bits()? as usize
        };

        Ok(Size(size_usize))
    }

    fn to_buf(&self, buf: &mut WriteByteBuffer) {
        let size_usize = self.0;

        if size_usize > 0b01111111 {
            true.to_buf(buf);
        } else {
            false.to_buf(buf);

            buf.write_remaining_bits(size_usize as u8)
                .expect("Should not fail as long as everything is aligned");
        }
    }
}
