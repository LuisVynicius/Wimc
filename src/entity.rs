use std::{fmt::Display, io::Error, path::PathBuf};

pub struct CommandLocation {
    pub path: PathBuf,
    pub lines: Vec<usize>
}

impl Display for CommandLocation {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "Path: {}\nlines: {:?}", self.path.display(), self.lines)

    }

}

pub struct FileError {
    pub path: String,
    error: Error
}

impl FileError {

    pub fn new(
        path: String,
        error: Error
    ) -> Self {

        Self {
            path,
            error
        }

    }

}

impl Display for FileError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "Path: {}\nError: {}", self.path, self.error)

    }

}