use std::env;
fn main() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments.");
        return;
    }

    for (index, arg) in env::args().enumerate() {
        println!("arguments are {arg} and index is {index}");
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {arg2}");
}
