use crate::{args::get_path_from_arg, commands::{find_commands, print_commands_location, print_errors}, file::get_file_paths};

mod args;
mod entity;
mod file;
mod commands;

fn main() {

    let path = get_path_from_arg();
    let paths= get_file_paths(&path);
    
    let (commands, errors) = find_commands(paths);

    print_commands_location(commands);
    print_errors(errors);
}