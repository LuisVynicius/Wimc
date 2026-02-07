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

use crate::entity::{CommandLocation, FileError};

pub fn get_file_paths(path_buf: &PathBuf) -> Vec<PathBuf> {

    let mut paths = vec![];

    let result = read_dir(path_buf);
    let dir = result.unwrap();

    for path_result in dir {
        let path = path_result.unwrap().path();

        if path.is_file() {
            paths.push(path);
        } else {
            get_file_paths(&path)
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