fn main() {
    let input = match std::fs::read_to_string("input") {
        Ok(i) => i.trim().to_string(),
        Err(e) => panic!("Error reading input {}", e),
    };

    let mut ints = Vec::new();

    for val in input.split(',') {
        ints.push(match val.parse::<usize>() {
            Ok(v) => v,
            Err(e) => panic!("Error parsing {}: {}", val, e),
        });
    }

    for i in (0..ints.len()).step_by(4) {
        if ints[i] == 99 {
            break;
        }
        let offset = ints[i + 3];
        match ints[i] {
            1 => ints[offset] = ints[ints[i + 1]] + ints[ints[i + 2]],
            2 => ints[offset] = ints[ints[i + 1]] * ints[ints[i + 2]],
            _ => panic!("Unexpected opcode {} at {}", ints[i], i),
        }
    }

    println!("{:?}", ints);
}
