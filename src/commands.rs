use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{
    entity::{CommandLocation, FileError},
    file::get_file,
};

pub fn find_commands(paths: Vec<PathBuf>, command: &str) -> (Vec<CommandLocation>, Vec<FileError>) {
    let mut commands = vec![];
    let mut errors = vec![];

    for path_buf in paths {
        match scan_file_for_commands(path_buf, command) {
            Ok(command_location_opt) => {
                if let Some(command_location) = command_location_opt {
                    commands.push(command_location);
                }
            }
            Err(error) => errors.push(error),
        }
    }

    (commands, errors)
}

fn scan_file_for_commands(
    path_buf: PathBuf,
    command: &str,
) -> Result<Option<CommandLocation>, FileError> {
    let mut count = 1usize;
    let mut command_location = CommandLocation {
        path: path_buf.clone(),
        lines: vec![],
    };

    let result = get_file(&path_buf);

    match result {
        Ok(file) => {
            let buf_reader = BufReader::new(file);

            for line_result in buf_reader.lines() {
                match line_result {
                    Ok(line) => {
                        if line.contains(&command) {
                            command_location.lines.push(count);
                        }
                    }
                    Err(error) => {
                        return Err(FileError::new(
                            path_buf.to_str().unwrap().to_string(),
                            error,
                        ));
                    }
                }

                count += 1;
            }
        }
        Err(error) => {
            let file_error = FileError::new(path_buf.to_str().unwrap().to_string(), error);

            return Err(file_error);
        }
    }

    match command_location.lines.len() > 0 {
        true => Ok(Some(command_location)),
        false => Ok(None),
    }
}
