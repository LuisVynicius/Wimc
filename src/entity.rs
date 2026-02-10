use std::{fmt::Display, io::Error, path::PathBuf};

pub struct CommandLocation {
    pub path: PathBuf,
    pub lines: Vec<usize>,
}

impl Display for CommandLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path: {}\nLines: {:?}", self.path.display(), self.lines)
    }
}

pub struct FileError {
    pub path: String,
    error: Error,
}

impl FileError {
    pub fn new(path: String, error: Error) -> Self {
        Self { path, error }
    }
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path: {}\nError: {}", self.path, self.error)
    }
}

pub struct FileResult {
    paths: Vec<PathBuf>,
    errors: Vec<FileError>,
}

impl FileResult {
    pub fn new() -> Self {
        Self {
            paths: vec![],
            errors: vec![],
        }
    }

    pub fn push_path(&mut self, path: PathBuf) {
        self.paths.push(path);
    }

    pub fn push_errors(&mut self, error: FileError) {
        self.errors.push(error);
    }

    pub fn get_paths(&self) -> &Vec<PathBuf> {
        &self.paths
    }

    pub fn get_errors(self) -> Vec<FileError> {
        self.errors
    }
}

pub struct Configs {
    all_paths: bool,
    all_extensions: bool,
    ocults: bool,
}

impl Configs {
    pub fn new(all_paths: bool, all_extensions: bool, ocults: bool) -> Self {
        Self {
            all_paths,
            all_extensions,
            ocults,
        }
    }

    pub fn get_all_paths(&self) -> bool {
        self.all_paths
    }

    pub fn get_all_extensions(&self) -> bool {
        self.all_extensions
    }

    pub fn get_ocults(&self) -> bool {
        self.ocults
    }
}

impl Default for Configs {
    fn default() -> Self {
        Self {
            all_extensions: false,
            all_paths: false,
            ocults: false,
        }
    }
}
