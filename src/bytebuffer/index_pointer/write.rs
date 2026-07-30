use crate::bytebuffer::index_pointer::IndexPointer;
use crate::ser_trait::{SerBytes, SerBytesStaticSized};
use bytebuffer::prelude::WriteByteBufferOwned;

pub trait IndexPointerWrite {
    /// Write the Ser type to the buffer and returns an [`IndexPointer`] at the location where the type was written.

    /// We don't need to restrict this method to [`SerBytesStaticSized`] because we don't care about the length of the content written as it's at the end of the buffer anyways.
    /// We only care when trying to write the data back at the [`IndexPointer`]

    fn write_with_index_pointer<S>(&mut self, val: &S) -> IndexPointer<S>
    where
        S: SerBytes;

    /// Writes data at the given [`IndexPointer`], if you need a method that accepts any type which implements [`SerBytes`],
    /// use the method [`WriteByteBufferOwned::try_write_at_index_pointer`]
    ///
    /// This function will not fail so long as all types which implement [`SerBytesStaticSized`] adhere to it's rules

    fn write_at_index_pointer<S>(&mut self, index_pointer: IndexPointer<S>, val: &S)
    where
        S: SerBytesStaticSized;

    /// Returns [`Err`] containing the length of the serialized type if the serialized type's length is not equal to the [`IndexPointer`] length
    fn try_write_at_index_pointer<S>(
        &mut self,
        index_pointer: IndexPointer<S>,
        val: &S,
    ) -> Result<(), usize>
    where
        S: SerBytes;
}

impl IndexPointerWrite for WriteByteBufferOwned {
    fn write_with_index_pointer<S>(&mut self, val: &S) -> IndexPointer<S>
    where
        S: SerBytes,
    {
        let index = self.buf().len();

        val.to_buf(self);

        let new_len = self.len();

        IndexPointer::new(index, new_len - index)
    }

    fn write_at_index_pointer<S>(&mut self, index_pointer: IndexPointer<S>, val: &S)
    where
        S: SerBytesStaticSized,
    {
        let mut temp_bb = WriteByteBufferOwned::new();
        val.to_buf(&mut temp_bb);

        self.buf_mut()[index_pointer.index..(index_pointer.index + index_pointer.len)]
            .copy_from_slice(temp_bb.buf());
    }

    fn try_write_at_index_pointer<S>(
        &mut self,
        index_pointer: IndexPointer<S>,
        val: &S,
    ) -> Result<(), usize>
    where
        S: SerBytes,
    {
        let mut temp_bb = WriteByteBufferOwned::new();
        val.to_buf(&mut temp_bb);

        if temp_bb.len() == index_pointer.len {
            self.buf_mut()[index_pointer.index..(index_pointer.index + index_pointer.len)]
                .copy_from_slice(temp_bb.buf());
            Ok(())
        } else {
            Err(temp_bb.len())
        }
    }
}
