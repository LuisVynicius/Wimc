use std::{
    fs::{File, OpenOptions, read_dir},
    io::{Error, Write},
    path::{Path, PathBuf},
};

use crate::{
    entity::{CommandLocation, Configs, FileError, FileResult},
    ignore::Ignored,
};

pub fn get_paths_by_root(
    file_result: &mut FileResult,
    ignored: &Ignored,
    path_buf: &PathBuf,
    configs: &Configs,
) {
    let root = match read_dir(path_buf) {
        Ok(read_dir) => read_dir,
        Err(_) => {
            println!("The root is unreachable");
            std::process::exit(1);
        }
    };

    for dir_entry_result in root {
        match dir_entry_result {
            Ok(dir_entry) => get_paths(file_result, ignored, dir_entry.path(), configs),
            Err(error) => {
                file_result.push_errors(FileError::new(path_buf.display().to_string(), error))
            }
        }
    }
}

fn get_paths(
    file_result: &mut FileResult,
    ignored: &Ignored,
    path_buf: PathBuf,
    configs: &Configs,
) {
    if path_buf.is_file() {
        if should_skip_file(&path_buf, ignored, configs) {
            return;
        }

        file_result.push_path(path_buf);
    } else {
        if should_skip_dir(&path_buf, ignored, configs) {
            return;
        }
        get_paths_by_path(file_result, ignored, &path_buf, configs);
    }
}

fn get_paths_by_path(
    file_result: &mut FileResult,
    ignored: &Ignored,
    path_buf: &PathBuf,
    configs: &Configs,
) {
    let dir = match read_dir(path_buf) {
        Ok(dir) => dir,
        Err(error) => {
            file_result.push_errors(FileError::new(path_buf.display().to_string(), error));
            return;
        }
    };

    for dir_entry_result in dir {
        match dir_entry_result {
            Ok(dir_entry) => {
                get_paths(file_result, ignored, dir_entry.path(), configs);
            }
            Err(error) => {
                FileError::new(path_buf.display().to_string(), error);
            }
        }
    }
}

fn should_skip_file(path: &Path, ignored: &Ignored, configs: &Configs) -> bool {
    let s = path.to_string_lossy();

    if s.contains("wimc_errors.txt") || s.contains("wimc_results.txt") {
        return true;
    }

    if !configs.get_all_extensions() {
        if let Some(ext) = path.extension() {
            if ignored.get_ignored_extensions().iter().any(|e| ext == *e) {
                return true;
            }
        }
    }

    false
}

fn should_skip_dir(path: &Path, ignored: &Ignored, configs: &Configs) -> bool {
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if !configs.get_ocults() && name.starts_with('.') {
        return true;
    }

    if !configs.get_all_paths() {
        let full = path.to_string_lossy();
        if ignored
            .get_ignored_files()
            .iter()
            .any(|ignored| full.contains(ignored))
        {
            return true;
        }
    }

    false
}

pub fn get_file(path: &PathBuf) -> Result<File, Error> {
    OpenOptions::new().read(true).open(path)
}

pub fn create_results_file(mut commands: Vec<CommandLocation>) {
    let result = create_file("wimc_results.txt");

    match result {
        Ok(mut file) => {
            if commands.len() == 0 {
                file.write_all("No results here".as_bytes()).unwrap();

                return;
            }

            commands.sort_by_key(|command_location| command_location.path.clone());

            let mut lines = 0usize;

            commands
                .iter()
                .for_each(|command_location| lines += command_location.lines.len());

            file.write_all(
                format!(
                    "Total_files: {} | Total_lines: {}\n\n",
                    commands.len(),
                    lines
                )
                .as_bytes(),
            )
            .unwrap();

            for command_location in commands {
                file.write_all(format!("{}\n\n", command_location).as_bytes())
                    .unwrap();
            }
        }
        Err(error) => panic!("Error to create wimc_results.txt: {error}"),
    }
}

pub fn create_errors_file(mut errors: Vec<FileError>) {
    let result = create_file("wimc_errors.txt");

    match result {
        Ok(mut file) => {
            if errors.len() == 0 {
                file.write_all("No errors here".as_bytes()).unwrap();

                return;
            }

            errors.sort_by_key(|file_error| file_error.path.clone());

            file.write_all(format!("Total_files: {}\n\n", errors.len()).as_bytes())
                .unwrap();

            for file_error in errors {
                file.write_all(format!("{}\n\n", file_error).as_bytes())
                    .unwrap();
            }
        }
        Err(error) => panic!("Error to create wimc_error.txt: {error}"),
    }
}

fn create_file(file_name: &'static str) -> Result<File, Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
}
