use std::borrow::Cow;
use std::fmt::{Display, Formatter};

mod bb_ref;
mod owned;
mod read_macro;

pub use bb_ref::*;
pub use owned::*;

pub type BBReadResult<T> = Result<T, ReadError<'static>>;

#[derive(Debug, Clone)]
pub enum SpecificError<'a> {
    U8,
    // U16,
    // U32,
    // U64,
    // U128,
    I8,
    // I16,
    // I32,
    // I64,
    // I128,
    // F32,
    // F64,
    Bytes { remaining_bytes: usize, got: usize },
    Bool,
    SingleBit,
    RemainingBits,
    Other(Cow<'a, str>),
}

/// An error that represents an inability to read or deserialize a type in some shape or form

#[derive(Debug, Clone)]
pub struct ReadError<'a> {
    /// The specific error generated from being deserialized, this is the value of the individual bytebuffer fail
    ///
    /// For example the bytes of a string.
    pub specific_error: SpecificError<'a>,
    /// The full type name which is being deserialized
    ///
    /// For example string
    of: Cow<'a, str>,
    /// If the value being deserialized is a subset of another
    ///
    /// For example elements of type S in a Vec<S>
    /// As in, if a Vec<S> fails to deserialized, this field should be Some with the read error of S
    child: Option<Box<Self>>,
}

impl<'a> ReadError<'a> {
    pub fn new(
        specific_error: SpecificError<'a>,
        of: impl Into<Cow<'a, str>>,
        child: Option<Self>,
    ) -> Self {
        Self {
            specific_error,
            of: of.into(),
            child: child.map(|child| Box::new(child)),
        }
    }

    pub fn new_parent(self, of: impl Into<Cow<'a, str>>) -> Self {
        Self::new(self.specific_error.clone(), of.into(), Some(self))
    }
}

impl<'a> Display for ReadError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error reading type: {:?} of {}",
            self.specific_error, self.of,
        )?;

        if let Some(child) = &self.child {
            writeln!(f, "; Error originates in child: {}", child)?;
        }

        Ok(())
    }
}

pub trait WithParent<'a> {
    fn with_parent(self, of: impl Into<Cow<'a, str>>) -> Self;
}

impl<'a, T> WithParent<'a> for Result<T, ReadError<'a>> {
    fn with_parent(self, of: impl Into<Cow<'a, str>>) -> Self {
        self.map_err(|read_error| read_error.new_parent(of))
    }
}

const DEFAULT_STR: &str = "Default";

impl<'a> Default for ReadError<'a> {
    fn default() -> Self {
        Self::new(SpecificError::Other(DEFAULT_STR.into()), DEFAULT_STR, None)
    }
}
