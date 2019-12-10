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

    // Use the min and max as bounds for our 2d vector of pairs (so really 3d)
    let mut grid = vec![vec![[0, 0]; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    // Determine where in vector center port is
    let center_x = (0 - min_x) as usize;
    let center_y = (0 - min_y) as usize;

    let mut wire = 0;
    for ref line in input.lines() {
        // Record each wire in vector, starting at center port
        let mut x = center_x;
        let mut y = center_y;
        let mut steps = 0;

        for ref seg in line.split(',') {
            match parse_direction(seg) {
                // Each grid sapce has two values, each value is the number of steps a wire took to get to this space, 0 means a wire isn't on that space
                Direction::Left(dist) => {
                    for i in 0..dist {
                        // Travel distance to the left
                        if grid[y][x - i][wire] == 0 {
                            // If the wire hasn't already been on the grid space, record how many steps it took to get here
                            grid[y][x - i][wire] = steps;
                        }
                        steps += 1;
                    }
                    x -= dist;
                }
                Direction::Right(dist) => {
                    for i in 0..dist {
                        if grid[y][x + i][wire] == 0 {
                            grid[y][x + i][wire] = steps;
                        }
                        steps += 1;
                    }
                    x += dist;
                }
                Direction::Up(dist) => {
                    for i in 0..dist {
                        if grid[y + i][x][wire] == 0 {
                            grid[y + i][x][wire] = steps;
                        }
                        steps += 1;
                    }
                    y += dist;
                }
                Direction::Down(dist) => {
                    for i in 0..dist {
                        if grid[y - i][x][wire] == 0 {
                            grid[y - i][x][wire] = steps;
                        }
                        steps += 1;
                    }
                    y -= dist;
                }
            };
        }

        if grid[y][x][wire] == 0 {
            grid[y][x][wire] = steps;
        }
        wire += 1;
    }

    let mut min_steps = usize::max_value();

    // Traverse entire grid (we could start and the center and go out but this is fast enough)
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let w1 = grid[i][j][0];
            let w2 = grid[i][j][1];
            // Check that both wires are on this grid space
            if w1 > 0 && w2 > 0 {
                // Record the number of steps each wire took to get here, remember the minimum
                let steps = w1 + w2;
                min_steps = min_steps.min(steps);
            }
        }
    }

    println!("{}", min_steps);
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
