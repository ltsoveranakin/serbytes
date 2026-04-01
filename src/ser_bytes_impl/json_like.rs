use crate::prelude::ReadError;
use ser_bytes_derive::SerBytes;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

mod serbytes {
    pub use crate::*;
}

// #[derive(SerBytes)]
pub struct Object(HashMap<String, JsonLikeValue>);

impl serbytes::prelude::SerBytes for Object {
    fn from_buf(
        buf: &mut serbytes::prelude::ReadByteBufferRefMut,
    ) -> serbytes::prelude::BBReadResult<Self> {
        let mut inner = || Ok(Object(serbytes::prelude::from_buf(buf)?));

        inner().map_err(|read_error: ReadError| {
            serbytes::prelude::ReadError::new(
                read_error.specific_error.clone(),
                "Object".into(),
                Some(read_error),
            )
        })
    }
    fn to_buf(&self, buf: &mut serbytes::prelude::WriteByteBufferOwned) {
        let Object(field0) = self;
        serbytes::prelude::to_buf(field0, buf);
    }
    fn size_hint() -> usize
    where
        Self: Sized,
    {
        0
    }
    fn approx_size(&self) -> usize {
        let Object(field0) = self;
        field0.approx_size()
    }
}

impl Deref for Object {
    type Target = HashMap<String, JsonLikeValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(SerBytes)]
pub enum JsonLikeValue {
    Object(Box<Object>),
    Array(Vec<JsonLikeValue>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}
