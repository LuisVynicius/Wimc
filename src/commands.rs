use std::{io::{BufRead, BufReader}, path::PathBuf};

use crate::{args::get_command_from_arg, entity::CommandLocation, file::get_file};

pub fn print_commands_location(mut commands: Vec<CommandLocation>) {

    commands.sort_by_key(
        |command_location| command_location.path.clone()
    );

    if commands.len() == 0 {
        println!("No results here :D");

        return;
    }

    for print in commands {
        println!("{print}");
    }

}

pub fn find_commands(paths: Vec<PathBuf>) -> Vec<CommandLocation> {

    let mut commands = vec![];

    for path_buf in paths {
        scan_file_for_commands(path_buf)
            .into_iter()
            .for_each(
                |command_location| commands.push(command_location)
            );
    }

    commands

}

pub fn scan_file_for_commands(path_buf: PathBuf) -> Option<CommandLocation> {

    let mut count = 1usize;
    let mut command_location = CommandLocation {
        path: path_buf.clone(),
        lines: vec![]
    };

    let file = get_file(&path_buf);
    let buf_reader = BufReader::new(file);
    let command = get_command_from_arg();

    for line in buf_reader.lines() {
        if line.unwrap().contains(&command) {
            command_location.lines.push(count);
        }

        count+=1;
    }

    match command_location.lines.len() > 0 {
        true => Some(command_location),
        false => None
    }

}