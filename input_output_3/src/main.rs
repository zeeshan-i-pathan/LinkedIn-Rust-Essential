use std::{fs, io::Write};
fn main() {
    let mut speech = String::new();
    speech += "We choose to go to the Moon in this decade\n";
    speech += "and do the other things,\n";
    speech += "not because they are easy,\n";
    speech += "but because they are hard.";
    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("planets.txt")
        .unwrap();
    file.write(b"\nPluto");
}
