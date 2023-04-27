#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: i8,
    propellant: f64,
}
fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };

    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };

    let vehicle3 = Shuttle { ..vehicle.clone() };
    vehicle.crew_size = 6;

    println!("Vehicle is {:?}", vehicle);
    println!("Vehicle 2 is {:?}", vehicle2);
    println!("Vehicle 3 is {:?}", vehicle3);
}
