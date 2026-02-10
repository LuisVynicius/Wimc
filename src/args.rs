use std::{env, path::PathBuf};

use crate::entity::Configs;

pub fn verify_args() {
    let args = env::args();

    if args.len() < 3 {
        println!("Must have at least 2 args");
        std::process::exit(0);
    }
}

pub fn get_args() -> (PathBuf, String, Configs) {
    let path_string = get_arg(1).unwrap();
    let command_string = get_arg(2).unwrap();

    let configs = get_extra_args(get_arg(3));

    (
        PathBuf::from(path_string),
        command_string,
        configs
    )
}

fn get_arg(nth: usize) -> Option<String> {
    let mut args = env::args();

    return args.nth(nth);
}

fn get_extra_args(arg_opt: Option<String>) -> Configs {
    let mut all_paths = false;
    let mut all_extensions = false;
    let mut ocults = false;

    match arg_opt {
        Some(arg) => {
            if !arg.starts_with("-") {
                println!("The extra args must start with \"-\"");
                std::process::exit(1)
            }

            let characters = arg.as_bytes();

            for &character in characters.iter() {
                match character {
                    45 => {}
                    // E
                    101 => all_extensions = true,
                    // D
                    100 => all_paths = true,
                    111 => ocults = true,
                    _ => {
                        println!("Extra args unknown: \"{}\"", character as char);
                        std::process::exit(1)
                    }
                }
            }

            Configs::new(all_paths, all_extensions, ocults)
        }
        None => Configs::default(),
    }
}
