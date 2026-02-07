use std::{io::{BufRead, BufReader}, path::PathBuf};

use crate::{args::get_command_from_arg, entity::{CommandLocation, FileError}, file::get_file};

pub fn print_commands_location(mut commands: Vec<CommandLocation>) {

    if commands.len() == 0 {
        println!("No results here :D");

        return;
    }

    commands.sort_by_key(
        |command_location| command_location.path.clone()
    );

    println!("Results:");

    for print in commands {
        println!("{print}");
    }

}

pub fn print_errors(errors: Vec<FileError>) {

    if errors.len() == 0 {
        println!("No errors here :D");

        return;
    }

    for error in errors {
        println!("{error}");
    }

}

pub fn find_commands(paths: Vec<PathBuf>) -> (Vec<CommandLocation>, Vec<FileError>) {

    let mut commands = vec![];
    let mut errors = vec![];

    for path_buf in paths {

        match scan_file_for_commands(path_buf) {
            Ok(command_location_opt) => {
                if let Some(command_location) = command_location_opt {
                    commands.push(command_location);
                }
            },
            Err(error) => errors.push(error),
        }
    }

    (commands, errors)

}

pub fn scan_file_for_commands(path_buf: PathBuf) -> Result<Option<CommandLocation>, FileError> {

    let mut count = 1usize;
    let mut command_location = CommandLocation {
        path: path_buf.clone(),
        lines: vec![]
    };

    let result = get_file(&path_buf);

    match result {
        Ok(file) => {
            let buf_reader = BufReader::new(file);
            let command = get_command_from_arg();

            for line in buf_reader.lines() {
                if line.unwrap().contains(&command) {
                    command_location.lines.push(count);
                }

                count+=1;
            }
        },
        Err(error) => {
            let file_error = FileError::new(path_buf.to_str().unwrap().to_string(), error);

            return Err(file_error);
        }
    }

    match command_location.lines.len() > 0 {
        true => Ok(Some(command_location)),
        false => Ok(None)
    }

}