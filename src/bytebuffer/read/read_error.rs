use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;

pub type BBReadResult<T> = Result<T, ReadError<'static>>;

/// An error that represents an inability to read or deserialize a type in some shape or form
///
/// Most of the time you can get away with a static lifetime here, it only exists for future proofing and custom implementations

#[derive(Debug, Clone)]
pub struct ReadError<'a> {
    /// The specific error generated from being deserialized, this is the value of the individual bytebuffer fail
    ///
    /// For example when reading a [String] first a [u16] (we'll call x) is read then based on the resulting number, that many bytes are read.
    /// If reading the x number of bytes fails then the [`SpecificError`] will be [`SpecificError::Bytes`]
    pub specific_error: SpecificError<'a>,
    /// The full type name which is being deserialized
    ///
    /// For example "String"
    of: Cow<'a, str>,
    /// If the value being deserialized is a subset of another
    ///
    /// For example elements of type S in a `Vec<S>`
    /// As in, if a `Vec<S>` fails to be deserialized, this field should be `Some` with the read error of S
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
            write!(f, "; Error originates in child:\n{}", child)?;
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
        Self::new(
            SpecificError::Other(Cow::Borrowed(DEFAULT_STR)),
            DEFAULT_STR,
            None,
        )
    }
}

impl Error for ReadError<'static> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.specific_error)
    }
}

impl From<ReadError<'static>> for io::Error {
    fn from(value: ReadError<'static>) -> Self {
        io::Error::new(ErrorKind::Other, Box::new(value))
    }
}

#[derive(Debug, Clone)]
pub enum SpecificError<'a> {
    U8,
    I8,
    Bytes { remaining_bytes: usize, got: usize },
    Bool,
    SingleBit,
    RemainingBits,
    Other(Cow<'a, str>),
}

impl<'a> Display for SpecificError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::U8 => "U8",

            Self::I8 => "I8",

            Self::Bytes {
                remaining_bytes,
                got,
            } => &format!("Bytes, remaining: {}; got: {}", remaining_bytes, got),

            Self::Bool => "Bool",

            Self::SingleBit => "SingleBit",

            Self::RemainingBits => "RemainingBits",

            Self::Other(other) => &format!("Other: {}", other),
        };

        write!(f, "SpecificError: {}", s)
    }
}

impl<'a> Error for SpecificError<'a> {}
