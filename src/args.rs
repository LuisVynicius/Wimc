use std::{env, path::PathBuf};

pub fn get_path_from_arg() -> PathBuf {
    let args = env::args();

    if args.len() != 2 {
        panic!("Deve ser inserído 1 argumento");
    }

    PathBuf::from(args.into_iter().nth(1).unwrap())
}