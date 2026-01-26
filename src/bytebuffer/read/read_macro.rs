macro_rules! read_ref_ty {
    ($t: ty, $call: ident, $size: literal) => {
        #[doc = concat!("Attempts to read a ", stringify!($t), " from the buffer")]
        #[doc = concat!("If there are not at least ", stringify!($size), " bytes in the buffer, it will return Err")]
        pub fn $call(&mut self) -> crate::bytebuffer::BBReadResult<$t> {
            use byteorder::ByteOrder;

            let bytes = crate::bytebuffer::ReadByteBufferRefMut::read_bytes_with_err_msg(self, $size, stringify!($t).into())?;

            Ok(byteorder::BigEndian::$call(bytes))
        }
    };
}

macro_rules! read_owned_ty {
    ($t: ty, $call: ident, $size: literal) => {
        #[doc = concat!("Attempts to read a ", stringify!($t), " from the buffer")]
        #[doc = concat!("If there are not at least ", stringify!($size), " bytes in the buffer, it will return Err")]
        pub fn $call(&mut self) -> crate::bytebuffer::BBReadResult<$t> {
            crate::bytebuffer::ReadByteBufferRefMut::$call(&mut crate::bytebuffer::ReadByteBufferOwned::rbb_ref_mut(self))
        }
    };
}

pub(crate) use {read_owned_ty, read_ref_ty};
