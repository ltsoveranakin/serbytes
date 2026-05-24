use crate::bytebuffer::ReadError;
use crate::ser_trait::SerBytes;
use std::io;
use std::path::Path;

pub type FromFileResult<'a, T> = Result<T, FromFileError<'a>>;

#[derive(Debug)]
pub enum FromFileError<'a> {
    ReadError(ReadError<'a>),
    IOError(io::Error),
}

pub trait SerBytesFs {
    fn from_file_path<'a>(path: impl AsRef<Path>) -> FromFileResult<'a, Self>
    where
        Self: Sized;

    fn write_to_file_path(&self, path: impl AsRef<Path>) -> io::Result<()>;
}

impl<T> SerBytesFs for T
where
    T: SerBytes,
{
    /// Loads and deserializes data from a given file path.
    ///
    /// Errors if it was unable to read bytes from the file.
    ///
    /// Errors if deserialization fails.

    fn from_file_path<'a>(path: impl AsRef<Path>) -> FromFileResult<'a, Self>
    where
        Self: Sized,
    {
        use std::fs;
        let buf = fs::read(path)?;

        Self::from_vec(buf).map_err(|read_error| FromFileError::ReadError(read_error))
    }

    /// Serializes and writes data to a given file path.
    /// If no parent directory exists, all necessary directories are created.
    ///
    /// Errors if it's unable to determine if a parent directory exists.
    ///
    /// Errors if an invalid path is given (a file path with no parent).
    ///
    /// Errors if it was unable to create all needed parent directories.
    fn write_to_file_path(&self, path: impl AsRef<Path>) -> io::Result<()> {
        use std::fs;
        if !fs::exists(&path)? {
            let parent_dir = if let Some(parent_dir) = path.as_ref().parent() {
                parent_dir
            } else {
                return Err(io::ErrorKind::InvalidFilename.into());
            };

            fs::create_dir_all(parent_dir)?;
        }

        let wbb = self.to_bb();

        fs::write(path, wbb.buf())
    }
}

impl<'a> From<io::Error> for FromFileError<'a> {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl<'a> From<ReadError<'a>> for FromFileError<'a> {
    fn from(value: ReadError<'a>) -> Self {
        Self::ReadError(value)
    }
}
