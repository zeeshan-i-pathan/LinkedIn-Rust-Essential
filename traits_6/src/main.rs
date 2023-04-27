use std::fmt;

fn get_displayable<T: fmt::Display>(a: T) -> impl fmt::Display {
    a
}
fn main() {
    println!("get_displayable is {}", get_displayable(13));
    println!("get_displayable is {}", get_displayable("Thirteen"));
}
