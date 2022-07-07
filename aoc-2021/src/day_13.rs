//! AOC Day xx
// use aoc_2021;
use log;
use regex::Regex;
use simple_logger::SimpleLogger;

use std::collections::VecDeque;

const EXAMPLE_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_13.txt"));

struct Instructions {
    /// The points
    points: Vec<(usize, usize)>,

    /// the axis and value of the fold
    folds: VecDeque<(char, usize)>,
}

impl Instructions {
    pub fn new(input: &str) -> Instructions {
        let points_re: Regex = Regex::new(r"(\d+),(\d+)").unwrap();
        let folds_re: Regex = Regex::new(r"fold along ([xy])=(\d+)").unwrap();

        let mut points = vec![];
        let mut folds = VecDeque::new();
        for line in input.split_terminator("\n") {
            if let Some(captures) = points_re.captures(line) {
                let cap_x = captures.get(1).expect("invalid regex somehow");
                let cap_y = captures.get(2).expect("invalid regex somehow");
                let x = cap_x.as_str().parse::<usize>().expect("invalid digit");
                let y = cap_y.as_str().parse::<usize>().expect("invalid digit");
                log::debug!("point {},{}", x, y);
                points.push((x, y));
            }

            if let Some(captures) = folds_re.captures(line) {
                let cap_axis = captures.get(1).expect("invalid regex somehow");
                let cap_mag = captures.get(2).expect("invalid regex somehow");
                let axis = cap_axis.as_str().chars().next().expect("invalid fold char");
                let mag = cap_mag.as_str().parse::<usize>().expect("invalid digit");
                log::debug!("fold along {}={}", axis, mag);
                folds.push_back((axis, mag));
            }
        }

        Instructions { points, folds }
    }

    pub fn bounding_box(&self) -> (usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in self.points.iter() {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        max_x += 1;
        max_y += 1;
        log::debug!("bounding box=({},{})", max_x, max_y);
        (max_x, max_y)
    }

    /// Fold along the first available fold, if there is a fold available.
    /// If folded, returns `true`, else `false`.
    pub fn fold(&mut self) -> bool {
        if let Some((axis, mag)) = self.folds.pop_front() {
            for (x, y) in self.points.iter_mut() {
                match axis {
                    'x' => {
                        if *x > mag {
                            let delta = *x - mag;
                            *x = *x - 2 * delta;
                        }
                    }
                    'y' => {
                        if *y > mag {
                            let delta = *y - mag;
                            *y = *y - 2 * delta;
                        }
                    }

                    _ => (),
                }
            }
            self.points.sort();
            self.points.dedup();
            return true;
        } else {
            return false;
        }
    }

    pub fn count(&self) -> usize {
        self.points.len()
    }
}

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (bound_x, bound_y) = self.bounding_box();
        let mut field = vec!['.'; bound_x * bound_y];

        // fill in the fold lines
        for (axis, value) in self.folds.iter() {
            if *axis == 'y' {
                for x in 0..bound_x {
                    field[bound_x * *value + x] = '-';
                }
            }
            if *axis == 'x' {
                for y in 0..bound_y {
                    field[bound_x * y + *value] = '|';
                }
            }
        }

        // fill in the points
        for (x, y) in self.points.iter() {
            field[*y * bound_x + *x] = 'X';
        }

        writeln!(f, "Instructions {{")?;
        for y in 0..bound_y {
            write!(f, "\t")?;
            for x in 0..bound_x {
                write!(f, "{}", field[y * bound_x + x])?;
            }
            write!(f, "\n")?;
        }
        writeln!(f, "\tcount={}", self.count())?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

fn part_one(input: &str) -> isize {
    let mut instructions = Instructions::new(input);
    log::debug!("{}", instructions);
    instructions.fold();
    log::debug!("{}", instructions);
    let count = instructions.count();
    // instructions.fold();
    // println!("{}", instructions);

    count as isize
}

fn part_two(input: &str) -> isize {
    let mut instructions = Instructions::new(input);
    let mut folded = true;
    while folded {
        log::debug!("{}", instructions);
        folded = instructions.fold();
    }
    log::debug!("{}", instructions);

    instructions.count() as isize
}

#[test]
fn example_part_one() {
    SimpleLogger::new().init().ok();
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 17);
}

#[test]
fn example_part_two() {
    SimpleLogger::new().init().ok();
    part_two(EXAMPLE_INPUT);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(706, result);
}

#[test]
fn test_part_two() {
    SimpleLogger::new().init().ok();
    part_two(OUR_INPUT.unwrap());
}
