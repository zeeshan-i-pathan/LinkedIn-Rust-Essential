#[derive(Debug)]
struct Rectangle<T>(T, T);
fn main() {
    let rect: Rectangle<i8> = Rectangle(1, 3);
    println!("rect is {:?}", rect);
}
