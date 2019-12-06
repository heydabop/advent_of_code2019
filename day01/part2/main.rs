use std::fs;

fn main() {
    let input = match fs::read_to_string("../input") {
        Ok(input) => input,
        Err(e) => panic!("Error reading file {}", e),
    };

    let mut fuel: i64 = 0;

    for line in input.lines() {
        let mass = match line.parse::<i64>() {
            Ok(mass) => mass,
            Err(e) => panic!("Error parsing int {}", e),
        };
        let mut mod_fuel = get_fuel(mass);
        let mut additional_fuel = get_fuel(mod_fuel);
        while additional_fuel > 0 {
            mod_fuel += additional_fuel;
            additional_fuel = get_fuel(additional_fuel);
        }
        fuel += mod_fuel;
    }

    println!("{}", fuel);
}

fn get_fuel(mass: i64) -> i64 {
    mass / 3 - 2
}
