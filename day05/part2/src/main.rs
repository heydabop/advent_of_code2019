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

    let mut i = 0;
    let mut digits = vec![0; 5]; //reusable buffer for opcodes and modes
    loop {
        for d in &mut digits {
            //clear buffer
            *d = 0;
        }
        get_digits(data[i], 0, &mut digits); // populate buffer with opcode and then mode digits
        let opcode = &digits[..2];
        let modes = &digits[2..];
        i = match opcode {
            // match on opcode
            [1, 0] => {
                // add
                add(&mut data, i, modes);
                i + 4
            }
            [2, 0] => {
                // multiply
                mul(&mut data, i, modes);
                i + 4
            }
            [3, 0] => {
                // input
                let offset = data[i + 1] as usize;
                data[offset] = input;
                i + 2
            }
            [4, 0] => {
                // output
                output(&data, i, modes);
                i + 2
            }
            [5, 0] => {
                // jump if true
                jit(&data, i, modes)
            }
            [6, 0] => {
                // jump if false
                jif(&data, i, modes)
            }
            [7, 0] => {
                // less than
                lt(&mut data, i, modes);
                i + 4
            }
            [8, 0] => {
                // equals
                eq(&mut data, i, modes);
                i + 4
            }
            [9, 9] => break,
            _ => panic!("Unexpected opcode {} at {}", data[i], i),
        };
    }

    println!("{:?}", data);
}

// Gets param values from `data` starting at `offset`
// `offset` should point to first param
// Will get a param for each paramater modes in `modes`
fn get_params(data: &[i64], offset: usize, modes: &[i64]) -> Vec<i64> {
    let mut params = Vec::new();
    for (i, &mode) in modes.iter().enumerate() {
        let mut param = data[offset + i];
        if mode == 0 {
            param = data[param as usize];
        }
        params.push(param);
    }
    params
}

fn add(data: &mut [i64], offset: usize, modes: &[i64]) {
    let params = get_params(&data, offset + 1, &modes[..2]); // get 2 params from data, starting at offset + 1
    let output = data[offset + 3] as usize;
    data[output] = params[0] + params[1];
}

fn mul(data: &mut [i64], offset: usize, modes: &[i64]) {
    let params = get_params(&data, offset + 1, &modes[..2]);
    let output = data[offset + 3] as usize;
    data[output] = params[0] * params[1];
}

fn output(data: &[i64], offset: usize, modes: &[i64]) {
    let params = get_params(&data, offset + 1, &modes[..1]); // get 1 param from data, starting at offset + 1
    println!("{}", params[0]);
}

// if first param is non-zero, return 2nd param as new offset, otherwise do nothing (return current offset)
fn jit(data: &[i64], offset: usize, modes: &[i64]) -> usize {
    let params = get_params(&data, offset + 1, &modes[..2]);

    if params[0] != 0 {
        params[1] as usize
    } else {
        offset + 3
    }
}

// if first param is zero, return 2nd param as new offset, otherwise advance as usual
fn jif(data: &[i64], offset: usize, modes: &[i64]) -> usize {
    let params = get_params(&data, offset + 1, &modes[..2]);

    if params[0] == 0 {
        params[1] as usize
    } else {
        offset + 3
    }
}

fn lt(data: &mut [i64], offset: usize, modes: &[i64]) {
    let params = get_params(&data, offset + 1, &modes[..2]);
    let output = data[offset + 3] as usize;
    data[output] = if params[0] < params[1] { 1 } else { 0 };
}

fn eq(data: &mut [i64], offset: usize, modes: &[i64]) {
    let params = get_params(&data, offset + 1, &modes[..2]);
    let output = data[offset + 3] as usize;
    data[output] = if params[0] == params[1] { 1 } else { 0 };
}

fn get_digits(d: i64, i: usize, v: &mut [i64]) {
    if d >= 10 {
        get_digits(d / 10, i + 1, v);
    }
    v[i] = d % 10;
}
