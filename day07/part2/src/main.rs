#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

fn main() {
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

    let mut max = i64::min_value();

    for perm in all_permutations(&mut [5, 6, 7, 8, 9], 5) {
        let mut amps = Vec::new();
        for phase in perm {
            amps.push(part2::Computer::new_with_input(data.clone(), &[phase]));
        }
        let mut output = vec![0];

        let mut i = 0;
        loop {
            if amps[i].halted() {
                break;
            }
            let mut input = Vec::new();
            input.append(&mut output);
            output = amps[i].run(&input);
            i += 1;
            i %= 5;
        }

        max = max.max(output[0]);
    }

    println!("{}", max);
}

fn all_permutations(v: &mut [i64], k: usize) -> Vec<Vec<i64>> {
    if k == 1 {
        return vec![Vec::from(v)];
    }
    let mut vv = all_permutations(v, k - 1);

    for i in 0..k - 1 {
        if k % 2 == 0 {
            v.swap(i, k - 1);
        } else {
            v.swap(0, k - 1);
        }
        let mut v2 = all_permutations(v, k - 1);
        vv.append(&mut v2);
    }

    vv
}
