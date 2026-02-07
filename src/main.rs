use std::{fmt::Display, fs::{File, OpenOptions, read_dir}, io::{BufRead, BufReader}, path::{Path, PathBuf}};

use crate::args::get_path_from_arg;

mod args;

fn main() {

    let path = get_path_from_arg();
    let paths= get_file_paths(&path);
    let prints = find_prints(paths);

    print_prints();

}

fn find_prints(paths: Vec<PathBuf>) -> Vec<PrintFounded> {

    let mut prints = vec![];

    for path_buf in paths {
        scan_file_for_prints(path_buf)
            .into_iter()
            .for_each(
                |print| prints.push(print)
            );
    }

    prints

}

fn scan_file_for_prints(path_buf: PathBuf) -> Option<PrintFounded> {

    let mut count = 1usize;
    let mut print = PrintFounded {
        path: path_buf.clone(),
        lines: vec![]
    };

    let file = get_file(&path_buf);
    let buf_reader = BufReader::new(file);
    let command = "println!";

    for line in buf_reader.lines() {

        if line.unwrap().contains(command) {
            print.lines.push(count);
        }

        count+=1;
    }

    match print.lines.len() > 0 {
        true => Some(print),
        false => None
    }

} 

fn get_file_paths(path_buf: &PathBuf) -> Vec<PathBuf> {

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

fn get_file(path: &PathBuf) -> File {
    OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap()
}

struct PrintFounded {
    path: PathBuf,
    lines: Vec<usize>
}

impl Display for PrintFounded {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "Path: {}:\nlines: {:?}", self.path.display(), self.lines)

    }

}