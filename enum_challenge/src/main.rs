enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64),
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Sorry I don't know this Location"),
            Location::Anonymous => println!("Sorry I can't tell you about this Location"),
            Location::Known(lat, lon) => {
                println!("The Latitude and Longitude for this location are {lat}, {lon}")
            }
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
