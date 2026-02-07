use std::{fmt::Display, path::PathBuf};

pub struct CommandLocation {
    pub path: PathBuf,
    pub lines: Vec<usize>
}

impl Display for CommandLocation {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "Path: {}\nlines: {:?}", self.path.display(), self.lines)

    }

}