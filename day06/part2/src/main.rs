use std::collections::HashMap;

fn main() {
    let input = match std::fs::read_to_string("input") {
        Ok(i) => i,
        Err(e) => panic!("Error reading file {}", e),
    };

    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let objs: Vec<&str> = line.split(')').collect();
        let target = String::from(objs[0]);
        let orbiter = String::from(objs[1]);

        match orbits.get_mut(&target) {
            Some(orbiters) => {
                orbiters.push(orbiter);
            }
            None => {
                orbits.insert(target, vec![orbiter]);
            }
        };
    }

    let you = find(&orbits, "COM", "YOU").unwrap();
    let san = find(&orbits, "COM", "SAN").unwrap();

    println!("{:?}\n{:?}", you, san);

    let mut matching_len = 0;

    for i in 0..you.len() {
        if you[i] != san[i] {
            matching_len = i;
            break;
        }
    }
    println!("{}", matching_len);

    println!("{}", you.len() - matching_len + san.len() - matching_len);
}

fn find(orbits: &HashMap<String, Vec<String>>, start: &str, finish: &str) -> Option<Vec<String>> {
    if let Some(orbiters) = orbits.get(start) {
        for orbiter in orbiters {
            if orbiter == finish {
                return Some(vec![String::from(finish)]);
            }
        }
        for orbiter in orbiters {
            if let Some(mut path) = find(orbits, orbiter, finish) {
                path.insert(0, String::from(start));
                return Some(path);
            }
        }
    }

    None
}
