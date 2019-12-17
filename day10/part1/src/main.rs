fn main() {
    // Read file and split by commas into vector of integers
    let file = match std::fs::read_to_string("input") {
        Ok(i) => i.trim().to_string(),
        Err(e) => panic!("Error reading input {}", e),
    };

    let map = part1::gen_map(&file);

    let (x, y, count) = part1::find_best_loc(&map);

    println!("{} {} {}", x, y, count);
}
