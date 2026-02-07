use std::{env, fs::{File, OpenOptions, read_dir}, io::{BufRead, BufReader, Read}, path::{Path, PathBuf}};

fn main() {
    let args = env::args();

    if args.len() != 2 {
        panic!("Deve ser inserído 1 argumento");
    }

    let path_string = args.into_iter().nth(1).unwrap();

    let path = Path::new(&path_string);

    let dir = read_dir(path);

    let mut prints = vec![];

    if let Ok(files) = dir {
        
        for file in files {
            let path = file.unwrap().path();

            let fs = get_file(&path);

            let buf_reader = BufReader::new(fs);

            let mut current_line = 0usize;

            let mut print_founded = PrintFounded {
                path: path,
                line: vec![]
            };

            for line in buf_reader.lines() {
                current_line+=1;
                
                if line.unwrap().contains("println!") {
                    print_founded.line.push(current_line);
                }
            }

            if print_founded.line.len() != 0 {
                prints.push(print_founded);
            }
        }

    }

    println!("{prints:?}");

}

fn find_print() {

}

fn get_file(path: &PathBuf) -> File {
    OpenOptions::new().read(true).open(path).unwrap()
}

#[derive(Debug)]
struct PrintFounded {
    path: PathBuf,
    line: Vec<usize>
}