use std::f64;
use std::f64::consts::PI;
use std::fmt;

struct Asteroid {
    x: usize,
    y: usize,
    angle: f64,
    dist: f64,
}

impl fmt::Debug for Asteroid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {:.3} {:.3}",
            self.x, self.y, self.angle, self.dist
        )
    }
}

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
                        if (angle2 - angle).abs() < f64::EPSILON {
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

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::similar_names)]
pub fn vaporize(map: &[Vec<bool>], x0: usize, y0: usize, count: usize) -> (usize, usize) {
    let mut asteroids = Vec::new();
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

            let yf = f64::from(y as u32);
            let xf = f64::from(x as u32);

            let angle = rotate((yf - y0f).atan2(xf - x0f));
            let dist = ((xf - x0f).powi(2) + (yf - y0f).powi(2)).sqrt();
            asteroids.push(Asteroid { x, y, angle, dist });
        }
    }

    asteroids.sort_by(|a, b| {
        if (b.angle - a.angle).abs() < f64::EPSILON {
            a.dist.partial_cmp(&b.dist).unwrap()
        } else {
            a.angle.partial_cmp(&b.angle).unwrap()
        }
    });

    let mut vaporized = 0;
    let mut last_angle = -1.0;
    'outer: loop {
        let mut to_remove = Vec::new();
        for (i, asteroid) in asteroids.iter().enumerate() {
            if (last_angle - asteroid.angle).abs() < f64::EPSILON {
                continue;
            }
            to_remove.push(i);
            vaporized += 1;
            if vaporized == count {
                break 'outer (asteroid.x, asteroid.y);
            }
            last_angle = asteroid.angle;
        }
        for &i in to_remove.iter().rev() {
            asteroids.remove(i);
        }
    }
}

fn rotate(angle: f64) -> f64 {
    if angle >= PI / -2.0 && angle <= PI {
        PI / 2.0 + angle
    } else if angle < PI / -2.0 && angle > -PI {
        PI * 5.0 / 2.0 + angle
    } else {
        panic!("Angle out of bounds: {}", angle)
    }
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
        assert_eq!((8, 2), vaporize(&map, 11, 13, 200));
    }

    #[test]
    fn rotation() {
        assert!((rotate(PI / -2.0)).abs() < f64::EPSILON);
        assert!((rotate(PI / -4.0) - PI * 1.0 / 4.0).abs() < f64::EPSILON);
        assert!((rotate(0.0) - PI / 2.0).abs() < f64::EPSILON);
        assert!((rotate(PI / 4.0) - PI * 3.0 / 4.0).abs() < f64::EPSILON);
        assert!((rotate(PI / 2.0) - PI).abs() < f64::EPSILON);
        assert!((rotate(PI * 3.0 / 4.0) - PI * 5.0 / 4.0).abs() < f64::EPSILON);
        assert!((rotate(PI) - PI * 3.0 / 2.0).abs() < f64::EPSILON);

        assert!((rotate(PI * -3.0 / 4.0) - PI * 7.0 / 4.0).abs() < f64::EPSILON);
        assert!((rotate(PI * -7.0 / 8.0) - PI * 13.0 / 8.0).abs() < f64::EPSILON);
    }
}
