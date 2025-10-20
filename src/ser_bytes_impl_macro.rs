macro_rules! ser_data_impl {
    ($t:ty, $call_signature:ident, $byte_size:tt) => {
        impl crate::prelude::SerBytes for $t {
            fn from_buf(buf: &mut bytebuffer::ByteBuffer) -> io::Result<Self> {
                paste::paste! {
                    buf.[<read_ $call_signature>]()
                }
            }

            fn to_buf(&self, buf: &mut bytebuffer::ByteBuffer) {
                paste::paste! {
                    buf.[<write_ $call_signature>](*self)
                }
            }
        }
    };
}

pub(crate) use ser_data_impl;
