struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}

trait Description {
    fn describe(&self) -> String {
        String::from("An Object Flying through Space")
    }
}

impl Description for Satellite {}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "SpaceStation {{name: \"{}\", crew_size: {}, altitude: {}}}",
            &self.name, &self.crew_size, &self.altitude
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };

    println!("Hubble is {}", hubble.describe());
    println!("ISS is {}", iss.describe());
}
