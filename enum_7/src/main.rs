fn main() {
    let number = Some(13);

    match number {
        Some(13) => println!("Thirteen"),
        _ => println!("Not Matched!"),
    }

    // Alternative in case of single match
    if let Some(13) = number {
        println!("Thirteen")
    }
}
