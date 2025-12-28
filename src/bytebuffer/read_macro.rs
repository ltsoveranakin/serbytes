macro_rules! read_ty {
    ($t: ty, $call: ident, $size: literal) => {
        #[doc = concat!("Attempts to read a ", stringify!($t), " from the buffer")]
        #[doc = concat!("If there are not at least ", stringify!($size), " bytes in the buffer, it will return Err")]
        pub fn $call(&mut self) -> crate::bytebuffer::BBReadResult<$t> {
            let bytes = crate::bytebuffer::ReadByteBuffer::read_bytes_with_err_msg(self, $size, stringify!($t).into())?;

            Ok(byteorder::BigEndian::$call(bytes))
        }
    };
}

pub(crate) use read_ty;
