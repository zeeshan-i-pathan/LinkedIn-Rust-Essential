use std::mem;
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };

    println!(
        "vehicle size on stack is : {} bytes",
        mem::size_of_val(&vehicle)
    );

    let boxed_vehicle = Box::<Shuttle>::new(vehicle);
    println!(
        "boxed_vehicle size on stack is {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );
    println!(
        "boxed_vehicle size on heap is {} bytes",
        mem::size_of_val(&*boxed_vehicle)
    );
    let unboxed_vehicle = *boxed_vehicle;
    println!(
        "unboxed_vehicle size on stack is {} bytes",
        mem::size_of_val(&unboxed_vehicle)
    );
}
