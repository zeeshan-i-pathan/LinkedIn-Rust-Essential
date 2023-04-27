#[derive(Debug)]
struct Rectangle<T, U>(T, U);

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.0
    }
}

impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        self.0 * 2 + self.1 * 2
    }
}
fn main() {
    let rect = Rectangle(1u8, 3u16);
    println!("rect is {:?}", rect);
    println!("Width is {}", rect.get_width());
    let rect = Rectangle(1u8, 3u8);
    println!("Perimeter is {}", rect.get_perimeter());
}
