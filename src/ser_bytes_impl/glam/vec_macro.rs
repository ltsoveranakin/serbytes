macro_rules! impl_for_vec {
    ($vec_ty:ty, $d_ty:ident, $dim:literal, $($field:ident),+) => {
        impl crate::prelude::SerBytes for $vec_ty {
            fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
                Ok(Self {
                    $(
                        $field: crate::prelude::from_buf(buf)?,
                    )+
                })
            }

            fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
                $(
                    crate::prelude::to_buf(buf, &self.$field);
                )+
            }

            #[inline(always)]
            fn size_hint() -> usize
            where
                Self: Sized,
            {
                $d_ty::size_hint() * $dim
            }

            #[inline(always)]
            fn approx_size(&self) -> usize {
                $d_ty::size_hint() * $dim
            }
        }

        impl crate::prelude::SerBytesStaticSized for $vec_ty {}
    };
}

macro_rules! impl_for_vec2 {
    ($vec_ty:ty, $d_ty:ident) => {
        crate::ser_bytes_impl::glam::vec_macro::impl_for_vec!($vec_ty, $d_ty, 2, x, y);
    };
}

macro_rules! impl_for_vec3 {
    ($vec_ty:ty, $d_ty:ident) => {
        crate::ser_bytes_impl::glam::vec_macro::impl_for_vec!($vec_ty, $d_ty, 3, x, y, z);
    };
}

macro_rules! impl_for_vec4 {
    ($vec_ty:ty, $d_ty:ident) => {
        crate::ser_bytes_impl::glam::vec_macro::impl_for_vec!($vec_ty, $d_ty, 4, x, y, z, w);
    };
}

pub(super) use {impl_for_vec, impl_for_vec2, impl_for_vec3, impl_for_vec4};
