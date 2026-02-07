use std::{
    fs::{
        File,
        OpenOptions,
        read_dir
    },
    io::{
        Error,
        Write
    },
    path::PathBuf
};

use crate::{entity::{CommandLocation, FileError}, ignore::Ignored};

pub fn get_file_paths(
    path_buf: &PathBuf,
    all_files: bool,
    all_extensions: bool
) -> Vec<PathBuf> {

    let mut paths = vec![];

    let result = read_dir(path_buf);
    let dir = result.unwrap();
    let ignored_files = Ignored::ignored_files();
    let ignored_extensions = Ignored::ignored_extensions();

    'path_result_for: for path_result in dir {
        let path = path_result.unwrap().path();

        if path.is_file() {
            if path.to_str().unwrap().contains("wimc_errors.txt") || path.to_str().unwrap().contains("wimc_results.txt") {
                continue 'path_result_for;
            }

            if !all_extensions {
                let extension_opt = path.extension();

                if let Some(extension) = extension_opt {
                    for &ignored_extension in &ignored_extensions {
                        if extension == ignored_extension {
                            continue 'path_result_for;
                        }
                    }
                }
            }

            paths.push(path);
        } else {
            if !all_files {
                for ignored_file in &ignored_files {
                    if path.to_str().unwrap().contains(ignored_file) {
                        continue 'path_result_for;
                    }
                }
            }

            get_file_paths(&path, all_files, all_extensions)
                .into_iter()
                .for_each(
                    |path| paths.push(path)
                );
        }
    }

    paths

}

pub fn get_file(path: &PathBuf) -> Result<File, Error> {
    
     OpenOptions::new()
        .read(true)
        .open(path)

}

pub fn create_results_file(mut commands: Vec<CommandLocation>) {

    let result = create_file("wimc_results.txt");

    match result {
        Ok(mut file) => {
            if commands.len() == 0 {
                file.write_all("No results here".as_bytes()).unwrap();

                return;
            }

            commands.sort_by_key(
                |command_location| command_location.path.clone()
            );

            let mut lines = 0usize;

            commands.iter()
                .for_each(
                    |command_location| lines+=command_location.lines.len()
                );
            
            file.write_all(format!("Total_files: {} | Total_lines: {}\n\n", commands.len(), lines).as_bytes()).unwrap();

            for command_location in commands {
                file.write_all(
                    format!("{}\n", command_location).as_bytes()
                ).unwrap();
            }
        },
        Err(error) => panic!("Error to create wimc_results.txt: {error}")
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

            errors.sort_by_key(
                |file_error| file_error.path.clone()
            );

            file.write_all(format!("Total_files: {}\n\n", errors.len()).as_bytes()).unwrap();

            for file_error in errors {
                file.write_all(
                    format!("{}\n", file_error).as_bytes()
                ).unwrap();
            }
        },
        Err(error) => panic!("Error to create wimc_error.txt: {error}")
    }

    

}

fn create_file(file_name: &'static str) -> Result<File, Error> {

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)

}