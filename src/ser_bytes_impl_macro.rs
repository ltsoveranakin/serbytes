macro_rules! ser_data_impl {
    ($t:ty, $call_signature:ident, $byte_size:literal) => {
        impl crate::prelude::SerBytes for $t {
            fn from_buf(
                buf: &mut crate::bytebuffer::ReadByteBuffer,
            ) -> crate::bytebuffer::Result<Self> {
                paste::paste! {
                    buf.[<read_ $call_signature>]()
                }
            }

            fn to_buf(&self, buf: &mut crate::bytebuffer::WriteByteBuffer) {
                paste::paste! {
                    buf.[<write_ $call_signature>](*self)
                }
            }
        }
    };
}

pub(crate) use ser_data_impl;
