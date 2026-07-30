pub mod write;

use crate::ser_trait::SerBytes;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
#[must_use = "If the index pointer is not needed for later operations, use the SerBytes::to_buf method on the type"]
pub struct IndexPointer<S>
where
    S: SerBytes,
{
    index: usize,
    len: usize,
    _s: PhantomData<S>,
}

impl<S> IndexPointer<S>
where
    S: SerBytes,
{
    /// 99% of cases you shouldn't be constructing this and should obtain it through the [`WriteByteBufferOwned::write_with_index_pointer`] method
    pub fn new(index: usize, len: usize) -> Self {
        Self {
            index,
            len,
            _s: PhantomData,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::index_pointer::write::IndexPointerWrite;
    use bytebuffer::prelude::{ReadByteBufferOwned, WriteByteBufferOwned};

    #[test]
    fn test_index_pointer() {
        let mut wbb = WriteByteBufferOwned::new();

        let test_i32 = 28954;
        let test_u64 = 8235213245;

        let i32_index_ptr = wbb.write_with_index_pointer(&test_i32);
        wbb.write_u64(test_u64);

        let new_i32 = 187452;

        wbb.write_at_index_pointer(i32_index_ptr, &new_i32);

        let mut dyn_sized = vec![10, 20, 30];

        let ip_dst = wbb.write_with_index_pointer(&dyn_sized);

        dyn_sized.push(60);

        wbb.try_write_at_index_pointer(ip_dst, &dyn_sized)
            .expect_err("Fail to write a different sized type to the buffer");

        let mut rbb = ReadByteBufferOwned::from_vec(wbb.into_vec());

        let i32_read_value = rbb.read_i32().expect("Read i32 from buf");
        let u64_read_value = rbb.read_u64().expect("Read u64 from buf");

        assert_ne!(test_i32, i32_read_value);

        assert_eq!(new_i32, i32_read_value);

        assert_eq!(test_u64, u64_read_value);
    }
}
