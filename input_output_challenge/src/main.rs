use std::{env, fs};

fn main() {
    if env::args().len() < 3 {
        eprintln!("Program requires two arguments: <file path> <search name>");
        std::process::exit(1);
    }

    let file_path = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line == search_name {
            println!("{} did walk on the Moon!", line);
            return;
        }
    }

    println!("{} did not walk on the Moon... ", search_name);
}
