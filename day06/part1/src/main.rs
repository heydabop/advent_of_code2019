use std::collections::HashMap;

fn main() {
    let input = match std::fs::read_to_string("input") {
        Ok(i) => i,
        Err(e) => panic!("Error reading file {}", e),
    };

    let mut orbits: HashMap<String, String> = HashMap::new();

    for line in input.lines() {
        let objs: Vec<&str> = line.split(')').collect();
        let target = objs[0];
        let orbiter = objs[1];

        orbits.insert(String::from(orbiter), String::from(target));
    }

    let mut num_orbits = 0;
    for mut target in orbits.values() {
        while orbits.contains_key(target) {
            num_orbits += 1;
            target = &orbits[target];
        }
        num_orbits += 1;
    }

    println!("{}", num_orbits);
}
