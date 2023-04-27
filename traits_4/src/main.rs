use std::{any, fmt};

fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {}", item, any::type_name::<T>());
}
fn main() {
    print_type(13);
    print_type(13.9);
    print_type("Thirteen");
    print_type([13]);
}
