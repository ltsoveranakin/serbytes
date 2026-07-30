macro_rules! read_ref_ty {
    ($t: ty, $call: ident, $size: literal) => {
        #[doc = concat!("Attempts to read a ", stringify!($t), " from the buffer")]
        #[doc = concat!("If there are not at least ", stringify!($size), " bytes in the buffer, it will return Err")]
        pub fn $call(&mut self) -> crate::prelude::BBReadResult<$t> {
            use byteorder::ByteOrder;

            let bytes = crate::prelude::WithParent::with_parent(crate::prelude::ReadByteBufferRefMut::read_bytes(self, $size), stringify!($t))?;

            Ok(byteorder::BigEndian::$call(bytes))
        }
    };
}

macro_rules! read_owned_ty {
    ($t: ty, $call: ident) => {
        #[doc = concat!("Shorthand for [`ReadByteBufferRefMut::", stringify!($call), "`]")]
        pub fn $call(&mut self) -> crate::prelude::BBReadResult<$t> {
            crate::prelude::ReadByteBufferRefMut::$call(
                &mut crate::prelude::ReadByteBufferOwned::rbb_ref_mut(self), /* usize */
            )
        }
    };
}

pub(crate) use {read_owned_ty, read_ref_ty};
