struct Size(usize);

// impl SerBytes for Size {
//     fn from_buf(buf: &mut ReadByteBuffer) -> std::io::Result<Self>
//     where
//         Self: Sized,
//     {
//         let first_byte = u8::from_buf(buf)?;
//
//         if (first_byte >> 7) & 1 == 1 {
//             let first_bit_trimmed = first_byte << 1;
//             let byte_first_trimmed = first_bit_trimmed >> 1;
//             Ok(Size(byte_first_trimmed as usize))
//         } else {
//             let leading_ones = first_byte.leading_ones();
//
//         }
//
//         // let takes_more_than_7_bits = from_buf::<bool>(buf)?;
//         //
//         // let size =
//         //
//         // if takes_more_than_7_bits {
//         //
//         // } else {
//         //     let bits = buf.read_remaining_bits()?;
//         //
//         //     bits
//         // };
//     }
//
//     fn to_buf(&self, buf: &mut WriteByteBuffer) {
//         todo!()
//     }
// }
