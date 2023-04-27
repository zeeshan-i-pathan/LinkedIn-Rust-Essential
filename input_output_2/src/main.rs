use std::fs;
fn main() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents are \n{contents}");
    for (index, line) in contents.lines().enumerate() {
        println!("{} - {line}", index + 1);
    }

    let contents = fs::read("planets.txt").unwrap();
    println!("contents are {:?}", contents)
}
