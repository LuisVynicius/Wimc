use std::{io::{BufRead, BufReader}, path::PathBuf};

use crate::{args::{get_command_from_arg, get_path_from_arg}, entity::CommandLocation, flie::{get_file, get_file_paths}};

mod args;
mod entity;
mod flie;

fn main() {

    let path = get_path_from_arg();
    let paths= get_file_paths(&path);
    let prints = find_commands(paths);

    print_commands_location(prints);

}

fn print_commands_location(mut prints: Vec<CommandLocation>) {

    prints.sort_by_key(
        |print_founder| print_founder.path.clone()
    );

    if prints.len() == 0 {
        println!("No results here :D");

        return;
    }

    for print in prints {
        println!("{print}");
    }

}

fn find_commands(paths: Vec<PathBuf>) -> Vec<CommandLocation> {

    let mut prints = vec![];

    for path_buf in paths {
        scan_file_for_commands(path_buf)
            .into_iter()
            .for_each(
                |print| prints.push(print)
            );
    }

    prints

}

fn scan_file_for_commands(path_buf: PathBuf) -> Option<CommandLocation> {

    let mut count = 1usize;
    let mut print = CommandLocation {
        path: path_buf.clone(),
        lines: vec![]
    };

    let file = get_file(&path_buf);
    let buf_reader = BufReader::new(file);
    let command = get_command_from_arg();

    for line in buf_reader.lines() {

        if line.unwrap().contains(&command) {
            print.lines.push(count);
        }

        count+=1;
    }

    match print.lines.len() > 0 {
        true => Some(print),
        false => None
    }

}