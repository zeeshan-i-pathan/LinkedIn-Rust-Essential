#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => 2.0 * r * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}
fn main() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_shape is {:?}", my_shape);

    println!("Perimeter is {}", my_shape.get_perimeter())
}
