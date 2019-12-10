use std::fs;

fn main() {
    let input = match fs::read_to_string("../input") {
        Ok(input) => input,
        Err(e) => panic!("Error reading file {}", e),
    };

    let mut fuel: u64 = 0;

    for line in input.lines() {
        let mass = match line.parse::<u64>() {
            Ok(mass) => mass,
            Err(e) => panic!("Error parsing int {}", e),
        };
        fuel += mass / 3 - 2;
    }

    println!("{}", fuel);
}
