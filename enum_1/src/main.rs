#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}
fn main() {
    let my_shape = Shape::Rectangle(10.0, 20.0);
    println!("my_shape is {:?}", my_shape)
}
