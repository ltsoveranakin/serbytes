macro_rules! read_ty {
    ($t: ty, $call: ident, $size: literal) => {
        #[doc = concat!("Attempts to read a ", stringify!($t), " from the buffer")]
        #[doc = concat!("If there are not at least ", stringify!($size), " bytes in the buffer, it will return None")]
        pub fn $call(&mut self) -> std::io::Result<$t> {
            let index = self.index;
            if !self.has_bytes_remaining($size) {
                None
            } else {
                self.index += $size;

                let slice = &mut self.buf[index..index + $size];
                Some(BigEndian::$call(slice))
            }
        }
    };
}

pub(crate) use read_ty;
