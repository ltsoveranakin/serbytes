macro_rules! write_ty {
    ($t: ty, $call: ident, $size: literal) => {
        #[doc = concat!("Writes a ", stringify!($t), " to the buffer")]
        pub fn $call(&mut self, n: $t) {
            self.bit_pos = 8;

            let mut new_slice = [0; $size];
            byteorder::BigEndian::$call(&mut new_slice, n);

            self.buf.extend(&new_slice);
        }
    };
}

pub(crate) use write_ty;
