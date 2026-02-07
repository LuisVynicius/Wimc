use std::{
    env,
    path::PathBuf
};

pub fn get_path_from_arg() -> PathBuf {

    let path_string = get_arg(1);
    
    PathBuf::from(path_string)

}

pub fn get_command_from_arg() -> String {

    let command_string = get_arg(2);

    command_string

}

fn get_arg(nth: usize) -> String {
    
    let mut args = env::args();

    if args.len() != 3 {
        println!("Must have exactly 2 args");
        std::process::exit(0);
    }

    return args.nth(nth).unwrap()

}