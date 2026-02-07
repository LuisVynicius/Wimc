use crate::{
    args::{get_args, verify_args},
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
mod ignore;

fn main() {

    verify_args();

    let (
        path,
        command,
        all_files,
        all_extensions
    ) = get_args();

    let paths= get_file_paths(&path, all_files, all_extensions);

    let (commands, errors) = find_commands(paths, &command);

    create_results_file(commands);
    create_errors_file(errors);

}