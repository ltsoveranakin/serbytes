mod vec_macro;

use crate::prelude::SerBytesStaticSized;
use crate::ser_bytes_impl::from_buf;
use crate::ser_bytes_impl::glam::vec_macro::{impl_for_vec2, impl_for_vec3, impl_for_vec4};
use crate::ser_trait::SerBytes;
use bytebuffer::prelude::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use glam::*;

// Vec2

impl_for_vec2!(I8Vec2, i8);
impl_for_vec2!(I16Vec2, i16);
impl_for_vec2!(IVec2, i32);
impl_for_vec2!(I64Vec2, i64);

impl_for_vec2!(U8Vec2, u8);
impl_for_vec2!(U16Vec2, u16);
impl_for_vec2!(UVec2, u32);
impl_for_vec2!(U64Vec2, u64);

impl_for_vec2!(Vec2, f32);
impl_for_vec2!(DVec2, f64);

// Vec3

impl_for_vec3!(I8Vec3, i8);
impl_for_vec3!(I16Vec3, i16);
impl_for_vec3!(IVec3, i32);
impl_for_vec3!(I64Vec3, i64);

impl_for_vec3!(U8Vec3, u8);
impl_for_vec3!(U16Vec3, u16);
impl_for_vec3!(UVec3, u32);
impl_for_vec3!(U64Vec3, u64);

impl_for_vec3!(Vec3, f32);
impl_for_vec3!(DVec3, f64);

// Vec4

impl_for_vec4!(I8Vec4, i8);
impl_for_vec4!(I16Vec4, i16);
impl_for_vec4!(IVec4, i32);
impl_for_vec4!(I64Vec4, i64);

impl_for_vec4!(U8Vec4, u8);
impl_for_vec4!(U16Vec4, u16);
impl_for_vec4!(UVec4, u32);
impl_for_vec4!(U64Vec4, u64);

impl_for_vec4!(DVec4, f64);

impl SerBytes for Vec4 {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self::new(
            from_buf(buf)?,
            from_buf(buf)?,
            from_buf(buf)?,
            from_buf(buf)?,
        ))
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.x.to_buf(buf);
        self.y.to_buf(buf);
        self.z.to_buf(buf);
        self.w.to_buf(buf);
    }

    #[inline(always)]
    fn size_hint() -> usize
    where
        Self: Sized,
    {
        f32::size_hint() * 4
    }

    #[inline(always)]
    fn approx_size(&self) -> usize {
        f32::size_hint() * 4
    }
}

impl SerBytesStaticSized for Vec4 {}
