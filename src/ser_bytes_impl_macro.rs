macro_rules! ser_data_impl {
    ($t:ty, $call_signature:ident, $byte_size:literal) => {
        impl crate::prelude::SerBytes for $t {
            fn from_buf(
                buf: &mut crate::bytebuffer::ReadByteBufferRefMut,
            ) -> crate::bytebuffer::BBReadResult<Self> {
                paste::paste! {
                    buf.[<read_ $call_signature>]()
                }
            }

            fn to_buf(&self, buf: &mut crate::bytebuffer::WriteByteBufferOwned) {
                paste::paste! {
                    buf.[<write_ $call_signature>](*self);
                }
            }

            fn size_hint() -> usize
            where
                Self: Sized,
            {
                $byte_size
            }

            fn approx_size(&self) -> usize {
                Self::size_hint()
            }
        }
    };
}

pub(crate) use ser_data_impl;
