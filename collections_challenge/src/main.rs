use std::{collections::HashMap, env, fs, io::ErrorKind, process};

fn main() {
    // Checking if the 1st argument is provided
    let file_name = match env::args().nth(1) {
        Some(v) => v,
        None => {
            println!("call with a file arguments e.g main from_the_earth_to_the_moon.txt");
            process::abort();
        }
    };

    // Checking if the argument is a valid file name
    let content = match fs::read_to_string(&file_name) {
        Ok(s) => s,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("File {file_name} not found");
                process::abort();
            }
            _ => process::abort(),
        },
    };

    // Hash Map to store count of each word
    let mut word_count: HashMap<String, u16> = HashMap::new();

    // Loop the content of the file by word and count it
    for word in content.split_whitespace() {
        let key = word
            .to_lowercase()
            .replace(",", "")
            .replace(".", "")
            .replace(":", "")
            .replace("\"", "")
            .replace("_", "")
            .replace("?", "")
            .replace(";", "")
            .replace("!", "")
            .replace("”", "")
            .replace(")", "")
            .replace("(", "")
            .replace("“", "")
            .replace("’s", "")
            .replace("’", "")
            .replace("‘", "");
        let counter = word_count.entry(key).or_insert(0);
        *counter += 1;
    }
    // Convert HashMap into a vector and sort it
    let mut count_vec: Vec<_> = word_count.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    match count_vec.get(0) {
        Some(v) => println!("The most frequently used word is \"{}\"", v.0),
        None => println!("No word found"),
    }
}
