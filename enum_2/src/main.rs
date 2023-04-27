#[derive(Debug)]
enum Shape {
    Circle(f64),             // radius
    Rectangle(f64, f64),     // width, height
    Triangle(f64, f64, f64), // three sides
}
fn main() {
    let my_shape = Shape::Rectangle(10.0, 20.0);
    println!("my_shape is {:?}", my_shape);
    match my_shape {
        Shape::Circle(r) => println!("Circle with radius {r}"),
        Shape::Rectangle(w, h) => println!("Rectangle with width : {w} x height : {h}"),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {a}, {b}, {c}"),
    }
}
