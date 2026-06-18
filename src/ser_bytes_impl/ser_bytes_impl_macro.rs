macro_rules! ser_data_impl {
    ($t:ty, $call_signature:ident, $byte_size:literal) => {
        impl crate::ser_trait::SerBytes for $t {
            #[inline(always)]
            fn from_buf(
                buf: &mut crate::bytebuffer::ReadByteBufferRefMut,
            ) -> crate::bytebuffer::BBReadResult<Self> {
                paste::paste! {
                    buf.[<read_ $call_signature>]()
                }
            }

            #[inline(always)]
            fn to_buf(&self, buf: &mut crate::bytebuffer::WriteByteBufferOwned) {
                paste::paste! {
                    buf.[<write_ $call_signature>](*self);
                }
            }

            #[doc = concat!("Size hint for a ", stringify!($t))]
            #[doc = ""]
            #[doc = concat!("This data takes up ", stringify!($byte_size), " byte(s) and as such this function will always return ", stringify!($byte_size))]
            #[inline(always)]
            fn size_hint() -> usize
            where
                Self: Sized,
            {
                $byte_size
            }

            #[inline(always)]
            fn approx_size(&self) -> usize {
                $byte_size
            }
        }

        impl crate::ser_trait::SerBytesStaticSized for $t {}
    };
}

macro_rules! ser_data_impl_u {
    ($t:ty, $call_signature:ident, $byte_size:literal) => {
        ser_data_impl!($t, $call_signature, $byte_size);

        impl crate::ser_bytes_impl::LengthLike for $t {
            #[inline(always)]
            fn from_usize(us: usize) -> Self {
                us as Self
            }

            #[inline(always)]
            fn to_usize(self) -> usize {
                self as usize
            }
        }
    };
}

pub(crate) use {ser_data_impl, ser_data_impl_u};
