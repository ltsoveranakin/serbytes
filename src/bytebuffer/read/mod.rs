mod bb_owned;
mod bb_peek;
mod bb_ref_mut;
mod read_error;
mod read_macro;

pub use bb_owned::*;
pub use bb_peek::*;
pub use bb_ref_mut::*;
pub use read_error::*;

pub trait ReadByteBuffer {
    fn read_bit(&mut self) -> BBReadResult<u8>;

    /// Resets the bit index to 0 and moves the head over to the next available byte

    fn flush_bits(&mut self);

    fn read_bits(&mut self, count: usize) -> BBReadResult<u8>;

    fn read_remaining_bits(&mut self) -> BBReadResult<u8>;

    fn read_bool(&mut self) -> BBReadResult<bool>;

    fn read_bytes(&mut self, size: usize) -> BBReadResult<&[u8]>;

    fn read_u8(&mut self) -> BBReadResult<u8>;
    fn read_u16(&mut self) -> BBReadResult<u16>;
    fn read_u32(&mut self) -> BBReadResult<u32>;
    fn read_u64(&mut self) -> BBReadResult<u64>;
    fn read_u128(&mut self) -> BBReadResult<u128>;

    fn read_i8(&mut self) -> BBReadResult<i8>;
    fn read_i16(&mut self) -> BBReadResult<i16>;
    fn read_i32(&mut self) -> BBReadResult<i32>;
    fn read_i64(&mut self) -> BBReadResult<i64>;
    fn read_i128(&mut self) -> BBReadResult<i128>;

    fn read_f32(&mut self) -> BBReadResult<f32>;
    fn read_f64(&mut self) -> BBReadResult<f64>;

    fn peek(&self) -> ReadByteBufferPeek<'_>;
}
