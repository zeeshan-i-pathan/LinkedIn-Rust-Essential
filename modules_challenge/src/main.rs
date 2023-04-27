use std::io;

use rand::Rng;

fn main() {
    let random_number: i8 = rand::thread_rng().gen_range(1..101);
    println!("I'm generating a secret...");
    println!("Enter your guess");
    loop {
        let mut buf = String::new();
        let number: i8 = match io::stdin().read_line(&mut buf) {
            Ok(_) => match buf.trim().parse::<i8>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Failed to convert to number");
                    continue;
                }
            },
            Err(_) => {
                println!("Failed to read line");
                continue;
            }
        };
        if number == random_number {
            println!("Match!");
            break;
        } else if number < random_number {
            println!("Less than secret! Guess again")
        } else {
            println!("Greater than secret! Guess again")
        }
    }
}
