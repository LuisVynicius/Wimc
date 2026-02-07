use std::{
    env,
    path::PathBuf
};

pub fn verify_args() {
    
    let args = env::args();

    if args.len() < 3 {
        println!("Must have at least 2 args");
        std::process::exit(0);
    }

}

pub fn get_args() -> (PathBuf, String, bool, bool) {

    let path_string = get_arg(1).unwrap();
    let command_string = get_arg(2).unwrap();
    
    let (
        all_paths,
        all_extensions
    ) = get_extra_args(get_arg(3));

    (
        PathBuf::from(path_string),
        command_string,
        all_paths,
        all_extensions
    )

}

fn get_arg(nth: usize) -> Option<String> {
    
    let mut args = env::args();

    return args.nth(nth)

}

fn get_extra_args(arg_opt: Option<String>) -> (bool, bool) {

    let mut all_paths = false;
    let mut all_extensions = false;

    match arg_opt {
        Some(arg) => {

            if !arg.starts_with("-") {
                println!("The extra args must start with \"-\"");
                std::process::exit(1)
            }

            let characters = arg.as_bytes();

            for &character in characters.iter() {
                
                if character != 45 && character != 102 && character != 101 {
                    println!("Extra args unknown: \"{}\"", character as char);
                    std::process::exit(1)
                }

                // Equal "E"
                if character == 101 {
                    all_extensions = true;
                }

                // Equal "F"
                if character == 102 {
                    all_paths = true;
                }

            }

            (
                all_paths,
                all_extensions
            )
        },
        None => (false, false)
    }

}