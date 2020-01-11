use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TextFileOutputError<'file_handling> {
    CannotCreate(io::Error),
    CannotWrite(io::Error),
    CannotFlush(io::Error),
    AlreadyExists(&'file_handling str),
}

impl<'file_handling> fmt::Display for TextFileOutputError<'file_handling> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TextFileOutputError::CannotCreate(ref error) => write!(f, "Cannot create error: {}", error),
            TextFileOutputError::CannotWrite(ref error) => write!(f, "Cannot write error: {}", error),
            TextFileOutputError::CannotFlush(ref error) => write!(f, "Cannot flush error: {}", error),
            TextFileOutputError::AlreadyExists(path) => {
                write!(f, "Already exists error: File {} is already exists", path)
            }
        }
    }
}
