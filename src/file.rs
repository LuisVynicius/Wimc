use std::{
    fs::{File, OpenOptions, read_dir},
    io::{Error, Write},
    path::PathBuf,
};

use crate::{
    entity::{CommandLocation, Configs, FileError, FileResult},
    ignore::Ignored,
};

pub fn get_paths_by_root(
    file_result: &mut FileResult,
    ignored: &Ignored,
    path_buf: &PathBuf,
    configs: &Configs
) {
    let result = read_dir(path_buf);

    let root = match result {
        Ok(read_dir) => read_dir,
        Err(_) => {
            println!("The root is unreachable");
            std::process::exit(1);
        }
    };

    for item_result in root {
        match item_result {
            Ok(item) => {
                let path = item.path().clone();

                if path.is_file() {
                    if path.to_str().unwrap().contains("wimc_errors.txt") ||
                        path.to_str().unwrap().contains("wimc_results.txt") {
                            continue;
                    }

                    if !configs.get_all_extensions() {
                        let extension_opt = path.extension();

                        if let Some(extension) = extension_opt {
                            if ignored.get_ignored_files()
                                .iter()
                                .any(|&ignored_extension| extension == ignored_extension)
                            {
                                continue;
                            }
                        }
                    }

                    file_result.push_path(item.path());
                } else {
                    get_paths_by_path(file_result, ignored, &path, configs);
                }
            },
            Err(error) => file_result.push_errors(FileError::new(path_buf.to_str().unwrap().to_string(), error)),
        }
    }
}

fn get_paths_by_path(
    file_result: &mut FileResult,
    ignored: &Ignored,
    path_buf: &PathBuf,
    configs: &Configs
) {
    
    let dir = match read_dir(path_buf) {
        Ok(dir) => dir,
        Err(error) => {
            file_result.push_errors(FileError::new(path_buf.to_str().unwrap().to_string(), error));
            return;
        }
    };

    for path_result in dir {
        let path = path_result.unwrap().path();

        if path.is_file() {
            if path.to_str().unwrap().contains("wimc_errors.txt")
                || path.to_str().unwrap().contains("wimc_results.txt")
            {
                continue;
            }

            if !configs.get_all_extensions() {
                let extension_opt = path.extension();

                if let Some(extension) = extension_opt {
                    if ignored.get_ignored_extensions()
                        .iter()
                        .any(|&ignored_extension| extension == ignored_extension)
                    {
                        continue;
                    }
                }
            }

            file_result.push_path(path);
        } else {
            let path_str = path.to_str().unwrap();

            if !configs.get_ocults() {
                if path_str.split('/').last().unwrap().starts_with(".") {
                    continue;
                }
            }
            
            if !configs.get_all_paths() {
                if ignored.get_ignored_files()
                    .iter()
                    .any(|&ignored_file| path_str.contains(ignored_file))
                {
                    continue;
                }
            }

            get_paths_by_path(
                file_result,
                ignored,
                &path,
                configs
            );
        }
    }
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
