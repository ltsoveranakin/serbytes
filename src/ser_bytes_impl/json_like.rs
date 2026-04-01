use ser_bytes_derive::SerBytes;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

mod serbytes {
    pub use crate::*;
}

#[derive(SerBytes)]
pub struct Object(HashMap<String, JsonLikeValue>);

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
