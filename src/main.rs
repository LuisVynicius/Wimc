use crate::{
    args::{get_args, verify_args},
    commands::find_commands,
    entity::FileResult,
    file::{create_errors_file, create_results_file, get_paths_by_root},
    ignore::Ignored,
};

mod args;
mod commands;
mod entity;
mod file;
mod ignore;

fn main() {
    verify_args();

    let (path, command, configs) = get_args();

    let mut file_result = FileResult::new();
    let ignored = Ignored::new();

    get_paths_by_root(&mut file_result, &ignored, &path, &configs);

    let commands = find_commands(&mut file_result, &command);

    create_results_file(commands);
    create_errors_file(file_result.get_errors());
}
