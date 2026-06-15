use crate::bytebuffer::{ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::prelude::{BBReadResult, SerBytes};

/// Work in progress type
pub struct Size(usize);

impl SerBytes for Size {
    fn from_buf(_buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        // let is_using_bit_count = buf.read_bool()?;
        //
        // let mut total_count = 1usize;
        // // first bit will always be 1
        // buf.read_bit()?;
        //
        // let mut bit = buf.read_bit()?;
        //
        // while bit == 1 {
        //     total_count |= (total_count << 1);
        //
        //     bit = buf.read_bit()?;
        // }
        //
        // let size = if is_using_bit_count {
        //     let bits_needed = 2usize.pow(total_count as u32);
        //     buf.read_bits(bits_needed)?
        // } else {
        //     let
        // };

        todo!()
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
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
