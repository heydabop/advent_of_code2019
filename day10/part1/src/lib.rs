pub fn gen_map(data: &str) -> Vec<Vec<bool>> {
    let mut map = Vec::new();

    for l in data.lines() {
        let mut row = Vec::new();
        for b in l.bytes() {
            row.push(match b {
                b'.' => false,
                b'#' => true,
                _ => panic!("Unrecognized char in input: {}", b),
            });
        }
        map.push(row);
    }

    map
}

#[allow(clippy::similar_names)]
pub fn find_best_loc(map: &[Vec<bool>]) -> (usize, usize, u64) {
    let mut max = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, asteroid) in row.iter().enumerate() {
            if !asteroid {
                continue;
            }
            let visible = count_visible(map, x, y);
            if visible > max {
                max = visible;
                max_x = x;
                max_y = y;
            }
        }
    }

    (max_x, max_y, max)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::similar_names)]
fn count_visible(map: &[Vec<bool>], x0: usize, y0: usize) -> u64 {
    let mut count = 0;
    let x0f = f64::from(x0 as u32);
    let y0f = f64::from(y0 as u32);

    for (y, row) in map.iter().enumerate() {
        for (x, asteroid) in row.iter().enumerate() {
            if !asteroid {
                continue;
            }
            if x == x0 && y == y0 {
                continue;
            }

            let y = f64::from(y as u32);
            let x = f64::from(x as u32);

            let angle = (y - y0f).atan2(x - x0f);
            let dist = ((x - x0f).powi(2) + (y - y0f).powi(2)).sqrt();

            let mut occluded = false;
            'occlusion: for (y2, row2) in map.iter().enumerate() {
                for (x2, asteroid2) in row2.iter().enumerate() {
                    // lots of loops, most efficient
                    if !asteroid2 {
                        continue;
                    }
                    if x2 == x0 && y2 == y0 {
                        continue;
                    }

                    let y2 = f64::from(y2 as u32);
                    let x2 = f64::from(x2 as u32);

                    let dist2 = ((x2 - x0f).powi(2) + (y2 - y0f).powi(2)).sqrt();
                    if dist2 < dist {
                        let angle2 = (y2 - y0f).atan2(x2 - x0f);
                        if (angle2 - angle).abs() < std::f64::EPSILON {
                            occluded = true;
                            break 'occlusion;
                        }
                    }
                }
            }
            if !occluded {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let data = ".#..#\n.....\n#####\n....#\n...##";
        let map = gen_map(&data);
        assert_eq!((3, 4, 8), find_best_loc(&map));
    }

    #[test]
    fn med() {
        let data = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let map = gen_map(&data);
        assert_eq!((5, 8, 33), find_best_loc(&map));
    }

    #[test]
    fn large() {
        let data = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        let map = gen_map(&data);
        assert_eq!((11, 13, 210), find_best_loc(&map));
    }
}
