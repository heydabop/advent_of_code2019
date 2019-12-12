#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use std::io::{self, prelude::*};

fn main() {
    // Read input integer
    let mut input = String::new();
    print!("Input ID: ");
    if let Err(e) = io::stdout().flush() {
        panic!(e);
    }
    if let Err(e) = io::stdin().read_line(&mut input) {
        panic!(e);
    }
    let input = match input.trim().parse::<i64>() {
        Ok(i) => i,
        Err(e) => panic!("Error parsing input: {}", e),
    };

    // Read file and split by commas into vector of integers
    let file = match std::fs::read_to_string("input") {
        Ok(i) => i.trim().to_string(),
        Err(e) => panic!("Error reading input {}", e),
    };

    let mut data = Vec::new();

    for val in file.split(',') {
        data.push(match val.parse::<i64>() {
            Ok(v) => v,
            Err(e) => panic!("Error parsing {}: {}", val, e),
        });
    }

    let mut c = part1::Computer::new(data);
    let output = c.run(Some(input));
    println!("{:?}", output);
    println!("{:?}", c.data());
}
