mod vec_macro;

use crate::bytebuffer::{BBReadResult, ReadByteBufferRefMut, WriteByteBufferOwned};
use crate::prelude::{SerBytes, SerBytesStaticSized, from_buf};
use glam::{IVec2, Vec2};

impl SerBytes for IVec2 {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self {
            x: from_buf(buf)?,
            y: from_buf(buf)?,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.x.to_buf(buf);
        self.y.to_buf(buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        u16::size_hint() * 2
    }

    fn approx_size(&self) -> usize {
        self.x.approx_size() + self.y.approx_size()
    }
}

impl SerBytesStaticSized for IVec2 {}

impl SerBytes for Vec2 {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self> {
        Ok(Self {
            x: from_buf(buf)?,
            y: from_buf(buf)?,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        self.x.to_buf(buf);
        self.y.to_buf(buf);
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        f32::size_hint() * 2
    }

    fn approx_size(&self) -> usize {
        self.x.approx_size() + self.y.approx_size()
    }
}

impl SerBytesStaticSized for Vec2 {}
