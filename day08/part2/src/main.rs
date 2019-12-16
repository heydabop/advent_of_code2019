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

    let mut final_image = vec![vec![2; width]; height];

    for layer in layers.iter().rev() {
        for (y, row) in layer.iter().enumerate() {
            for (x, &p) in row.iter().enumerate() {
                if p != 2 {
                    final_image[y][x] = p;
                }
            }
        }
    }

    for row in final_image {
        for p in row {
            if p == 0 {
                print!(" ");
            } else if p == 1 {
                print!("\u{2588}");
            }
        }
        println!();
    }
}
