//! AOC Day xx
// use aoc_2021;
use ansi_term;

const EXAMPLE_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_15.txt"));

struct CaveRisk<const X: usize, const Y: usize> {
    risk: [[u8; Y]; X],
    distance: [[usize; Y]; X],
    target: (usize, usize),
}

const MOVE_DELTAS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl<const X: usize, const Y: usize> CaveRisk<X, Y> {
    pub fn new(input: &str) -> Box<Self> {
        let mut risk = [[0; Y]; X];

        for (ind_y, line) in input.split_terminator('\n').enumerate() {
            for (ind_x, c) in line.chars().enumerate() {
                risk[ind_x][ind_y] = c.to_digit(10).unwrap() as u8;
            }
        }

        Self::new_from_map(&risk)
    }

    pub fn new_from_map(map: &[[u8; Y]; X]) -> Box<Self> {
        let mut cave = Box::new(CaveRisk {
            risk: *map,
            distance: [[usize::MAX; Y]; X],
            target: (X - 1, Y - 1),
        });

        cave.distance[0][0] = 0; // starting location has no distance

        cave
    }

    pub fn compute(&mut self) {
        for y in 0..Y {
            for x in 0..X {
                // visit all neighbors
                for (dx, dy) in MOVE_DELTAS.iter() {
                    let px = x as isize + dx;
                    let py = y as isize + dy;
                    // ensure the point is valid
                    if px < 0 || px >= X as isize || py < 0 || py >= Y as isize {
                        continue;
                    }

                    // compute potential new distance. Use if it's shorter than existing
                    let px = px as usize;
                    let py = py as usize;
                    let distance = self.distance[x][y] + self.risk[px][py] as usize;
                    if distance < self.distance[px][py] {
                        self.distance[px][py] = distance;
                    }
                }
            }
        }
    }
}

impl<const X: usize, const Y: usize> std::fmt::Display for CaveRisk<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // calculate the most efficient path, if it is fully computed
        let mut position = self.target;
        let mut path = vec![];
        if self.distance[position.0][position.1] != usize::MAX {
            path.push(position);
            loop {
                let mut least_expensive = usize::MAX;
                let mut next_position = (0, 0);

                if position == (0, 0) {
                    break;
                }

                for (dx, dy) in MOVE_DELTAS.iter() {
                    let px = (position.0 as isize + dx) as usize;
                    let py = (position.1 as isize + dy) as usize;
                    // ensure the point is valid
                    if px >= X || py >= Y {
                        continue;
                    }
                    if self.distance[px][py] < least_expensive {
                        least_expensive = self.distance[px][py];
                        next_position = (px, py);
                    }
                }

                if least_expensive == usize::MAX {
                    break;
                }
                path.push(next_position);
                position = next_position;
            }
        }

        for y in 0..Y {
            for x in 0..X {
                let mut c = ansi_term::Style::new();
                if self.distance[x][y] == usize::MAX {
                    c = c.dimmed();
                }
                if path.contains(&(x, y)) {
                    c = c.bold();
                }
                write!(f, "{}", c.paint(format!("{}", self.risk[x][y])))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn expand_map<const X: usize, const Y: usize, const EX: usize, const EY: usize>(
    input: &str,
) -> [[u8; EY]; EX] {
    let mut risk: [[u8; EY]; EX] = [[0; EY]; EX];

    // ensure the expanded map is a multiple of the original
    assert!(EY % Y == 0);
    assert!(EX % X == 0);

    for (ind_y, line) in input.split_terminator('\n').enumerate() {
        for (ind_x, c) in line.chars().enumerate() {
            risk[ind_x][ind_y] = c.to_digit(10).unwrap() as u8;
        }
    }

    for dy in 0..(EY / Y) {
        for dx in 0..(EX / X) {
            for y in 0..Y {
                for x in 0..X {
                    let px = x + dx * X;
                    let py = y + dy * Y;
                    risk[px][py] = risk[x][y] + (dy + dx) as u8;
                    while risk[px][py] > 9 {
                        risk[px][py] -= 9;
                    }
                }
            }
        }
    }

    risk
}

fn part_one<const X: usize, const Y: usize>(input: &str) -> isize {
    let mut cave = CaveRisk::<X, Y>::new(input);

    cave.compute();

    print!("{}", cave);

    let (tx, ty) = cave.target;
    cave.distance[tx as usize][ty as usize] as isize
}

fn part_two<const X: usize, const Y: usize, const EX: usize, const EY: usize>(
    input: &str,
) -> isize {
    let map: [[u8; EY]; EX] = expand_map::<X, Y, EX, EY>(input);
    // for y in 0..EY {
    //     for x in 0..EX {
    //         print!("{}", map[x][y]);
    //     }
    //     print!("\n");
    // }

    let mut cave = CaveRisk::<EX, EY>::new_from_map(&map);

    cave.compute();

    print!("{}", cave);

    let (tx, ty) = cave.target;

    cave.distance[tx as usize][ty as usize] as isize
}

#[test]
fn example_part_one() {
    let result = part_one::<10, 10>(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 40);
}

#[test]
fn example_part_two() {
    let result = part_two::<10, 10, 50, 50>(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 315);
}

#[test]
fn test_part_one() {
    let result = part_one::<100, 100>(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(741, result);
}

#[test]
fn test_part_two() {
    let result = part_two::<100, 100, 500, 500>(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    more_asserts::assert_lt!(2990, result);
    more_asserts::assert_gt!(2900, result);
}
