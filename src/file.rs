use std::{fs::{File, OpenOptions, read_dir}, path::PathBuf};

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

pub fn get_file(path: &PathBuf) -> File {
    
    OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap()

}