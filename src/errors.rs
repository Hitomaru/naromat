use std::fmt;
use std::io;
use std::path::Path;

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

#[derive(Debug)]
pub enum FileProcessError<'file_process> {
    InvalidPath(InvalidPathError<'file_process>),
    SaveError(TextFileOutputError<'file_process>),
    LoadError(Box<dyn std::error::Error>),
}

impl<'file_process> fmt::Display for FileProcessError<'file_process> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File process failed")
    }
}

#[derive(Debug)]
pub enum DirectoryProcessError<'file_process> {
    InvalidPath(InvalidPathError<'file_process>),
    SaveError(TextFileOutputError<'file_process>),
    LoadError(Box<dyn std::error::Error>),
    Other,
}

impl<'file_process> fmt::Display for DirectoryProcessError<'file_process> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File process failed")
    }
}

#[derive(Debug)]
pub enum InvalidPathError<'file_handling> {
    FileNotFound(&'file_handling Path),
}

impl<'file_handling> fmt::Display for InvalidPathError<'file_handling> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidPathError::FileNotFound(path) => write!(f, "Not exists error: File {} is not found", path.display()),
        }
    }
}
