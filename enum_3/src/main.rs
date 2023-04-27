fn main() {
    let my_number = 4_u8;
    let result = match my_number {
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => {
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("Result is {result}")
}
