enum Direction {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
}

fn main() {
    let input = match std::fs::read_to_string("../input") {
        Ok(i) => i.trim().to_string(),
        Err(e) => panic!("Error reading input {}", e),
    };

    let mut max_x: i64 = 0;
    let mut min_x: i64 = 0;
    let mut max_y: i64 = 0;
    let mut min_y: i64 = 0;

    // Chart the x and y of both wires, tracking the min and max of each
    for ref line in input.lines() {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        for ref seg in line.split(',') {
            match parse_direction(seg) {
                Direction::Left(dist) => {
                    x -= dist as i64;
                    min_x = min_x.min(x);
                }
                Direction::Right(dist) => {
                    x += dist as i64;
                    max_x = max_x.max(x);
                }
                Direction::Up(dist) => {
                    y += dist as i64;
                    max_y = max_y.max(y);
                }
                Direction::Down(dist) => {
                    y -= dist as i64;
                    min_y = min_y.min(y);
                }
            };
        }
    }

    println!("{} {} {} {}", max_x, min_x, max_y, min_y);

    // Use the min and max as bounds for our 2d vector
    let mut grid = vec![vec![0; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    // Determine where in vector center port is
    let center_x = (0 - min_x) as usize;
    let center_y = (0 - min_y) as usize;

    let mut wire = 1;
    for ref line in input.lines() {
        // Record each wire in vector, starting at center port
        let mut x = center_x;
        let mut y = center_y;
        for ref seg in line.split(',') {
            match parse_direction(seg) {
                // A grid space will be either 0, 1, 2, or 3
                // 0 - no wires
                // 1 - first wire
                // 2 - second wire
                // 3 - both wires
                Direction::Left(dist) => {
                    for i in 0..dist {
                        // Travel distance to the right
                        if grid[y][x - i] < wire {
                            // If grid space is less than our wire value we haven't visited it as this wire yet, increment value
                            grid[y][x - i] += wire;
                        }
                    }
                    x -= dist;
                }
                Direction::Right(dist) => {
                    for i in 0..dist {
                        if grid[y][x + i] < wire {
                            grid[y][x + i] += wire;
                        }
                    }
                    x += dist;
                }
                Direction::Up(dist) => {
                    for i in 0..dist {
                        if grid[y + i][x] < wire {
                            grid[y + i][x] += wire;
                        }
                    }
                    y += dist;
                }
                Direction::Down(dist) => {
                    for i in 0..dist {
                        if grid[y - i][x] < wire {
                            grid[y - i][x] += wire;
                        }
                    }
                    y -= dist;
                }
            };
        }
        wire += 1;
    }

    let mut min_dist = usize::max_value();

    // Traverse entire grid (we could start and the center and go out but this is fast enough)
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // Calculate distances for grid spaces that had both wires on them and aren't the center, record the minimum
            if (i != center_y || j != center_x) && grid[i][j] == 3 {
                let dist = (center_y as i64 - i as i64).abs() + (center_x as i64 - j as i64).abs();
                min_dist = min_dist.min(dist as usize);
            }
        }
    }

    println!("{}", min_dist);
}

fn parse_direction(s: &str) -> Direction {
    let distance = match s[1..].parse::<usize>() {
        Ok(d) => d,
        Err(e) => panic!("Error parsing {}: {}", s, e),
    };
    match s.chars().nth(0) {
        Some('L') => Direction::Left(distance),
        Some('R') => Direction::Right(distance),
        Some('U') => Direction::Up(distance),
        Some('D') => Direction::Down(distance),
        Some(_) => panic!("Unknown direction from string: {}", s),
        None => panic!("Missing direction from string: {}", s),
    }
}
