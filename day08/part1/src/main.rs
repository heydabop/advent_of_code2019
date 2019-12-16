fn main() {
    // Read file and split by commas into vector of integers
    let file = match std::fs::read_to_string("input") {
        Ok(i) => i.trim().to_string(),
        Err(e) => panic!("Error reading input {}", e),
    };

    let mut data = Vec::new();

    for b in file.bytes() {
        data.push(b - 48);
    }

    let mut layers = Vec::new();
    let width = 25;
    let height = 6;

    loop {
        let mut layer = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(data[layers.len() * width * height + y * width + x]);
            }
            layer.push(row);
        }
        layers.push(layer);
        if layers.len() * width * height == data.len() {
            break;
        }
    }

    let mut min = i64::max_value();
    let mut min_layer = 0;

    for (i, layer) in layers.iter().enumerate() {
        let mut zeroes = 0;
        for row in layer {
            for &p in row {
                if p == 0 {
                    zeroes += 1;
                }
            }
        }
        if zeroes < min {
            min = zeroes;
            min_layer = i;
        }
    }

    let mut ones = 0;
    let mut twos = 0;

    for row in &layers[min_layer] {
        for &p in row {
            if p == 1 {
                ones += 1;
            } else if p == 2 {
                twos += 1;
            }
        }
    }

    println!("{}", ones * twos);
}
