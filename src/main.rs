use crate::{
    args::get_path_from_arg,
    commands::find_commands,
    file::{
        create_errors_file,
        create_results_file,
        get_file_paths
    }
};

mod args;
mod entity;
mod file;
mod commands;

fn main() {

    let path = get_path_from_arg();
    let paths= get_file_paths(&path);
    
    let (commands, errors) = find_commands(paths);

    create_results_file(commands);
    create_errors_file(errors);

}