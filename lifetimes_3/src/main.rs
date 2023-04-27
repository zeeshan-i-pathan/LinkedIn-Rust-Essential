fn best_fuel<'a>(x: &'a str, y: &str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn main() {
    let result;
    let propellant_1 = String::from("RP-1");
    {
        let propellant_2 = String::from("LNG");
        result = best_fuel(&propellant_1, &propellant_2);
    }
    println!("best_fuel is {}", result);
}
