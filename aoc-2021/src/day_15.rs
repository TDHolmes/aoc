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
    position: (isize, isize),
    target: (isize, isize),
    cost: usize,
    path: Vec<(isize, isize)>,
}

/// The risk of an average block
const AVERAGE_RISK: usize = 10;
const MOVE_DELTAS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl<const X: usize, const Y: usize> CaveRisk<X, Y> {
    pub fn new(input: &str) -> Self {
        let mut cave = CaveRisk {
            risk: [[0; Y]; X],
            position: (0, 0),
            target: (X as isize - 1, Y as isize - 1),
            cost: 0,
            path: vec![(0, 0)],
        };
        for (ind_y, line) in input.split_terminator('\n').enumerate() {
            for (ind_x, c) in line.chars().enumerate() {
                cave.risk[ind_x][ind_y] = c.to_digit(10).unwrap() as u8;
            }
        }

        cave
    }

    fn get_risk<T>(&self, pos: (T, T)) -> u8
    where
        T: TryInto<usize>,
    {
        let x = pos.0.try_into().unwrap_or(usize::MAX);
        let y = pos.1.try_into().unwrap_or(usize::MAX);

        self.risk[x][y]
    }

    /// Moves our [position] one step forward towards our [target]
    ///
    /// # Return
    /// true if we've reached the end
    pub fn step(&mut self) -> bool {
        // Check if we've already arrived
        if self.position == self.target {
            return true;
        }

        let mut best_pos = (isize::MAX, isize::MAX);
        let mut best_pos_cost = isize::MAX;
        let mut first = (-1, -1);
        let mut second = (-1, -1);
        let mut third = (-1, -1);
        for (dx, dy) in MOVE_DELTAS.iter() {
            let first_pend = (
                self.position.0 as isize + *dx,
                self.position.1 as isize + *dy,
            );
            // ensure we're in bounds
            if first_pend.0 < 0
                || first_pend.1 < 0
                || first_pend.0 >= X as isize
                || first_pend.1 >= Y as isize
            {
                continue;
            }
            let first_cost = if first_pend == self.target {
                isize::MIN
            } else {
                self.get_risk(first_pend) as isize
            };

            for (d2x, d2y) in MOVE_DELTAS.iter() {
                let second_pend = (first_pend.0 as isize + *d2x, first_pend.1 as isize + *d2y);
                // ensure we're in bounds
                if second_pend.0 < 0
                    || second_pend.1 < 0
                    || second_pend.0 >= X as isize
                    || second_pend.1 >= Y as isize
                {
                    continue;
                }
                if second_pend == self.position {
                    continue;
                }
                let second_cost = self.get_risk(second_pend) as isize;

                for (d3x, d3y) in MOVE_DELTAS.iter() {
                    let third_pend = (second_pend.0 as isize + *d3x, second_pend.1 as isize + *d3y);
                    // ensure we're in bounds
                    if third_pend.0 < 0
                        || third_pend.1 < 0
                        || third_pend.0 >= X as isize
                        || third_pend.1 >= Y as isize
                    {
                        continue;
                    }
                    if third_pend == self.position || third_pend == first {
                        continue;
                    }

                    let third_cost = self.get_risk(third_pend) as isize;
                    let total_cost = first_cost
                        + second_cost
                        + third_cost
                        + self.manhattan_distance(third_pend) as isize * AVERAGE_RISK as isize;

                    // println!(
                    //     "Testing third {},{} (cost={})",
                    //     third_pend.0, third_pend.1, third_cost
                    // );

                    // println!(
                    //     "cost {} + {} + {} + {} = {}",
                    //     first_cost,
                    //     second_cost,
                    //     third_cost,
                    //     self.manhattan_distance(third) * AVERAGE_RISK as usize,
                    //     total_cost
                    // );

                    if total_cost < best_pos_cost && !self.path.contains(&first_pend) {
                        best_pos = first_pend;
                        best_pos_cost = total_cost;
                        first = first_pend;
                        second = second_pend;
                        third = third_pend;
                    }
                }
            }
        }

        // self.illustrate_path(vec![first, second, third]);

        // println!(
        //     "Moving from {},{} to {},{}",
        //     self.position.0, self.position.1, best_pos.0, best_pos.1
        // );
        self.position = best_pos;
        self.cost += self.get_risk(self.position) as usize;
        self.path.push(self.position);

        // check if we've made it
        if self.position == self.target {
            return true;
        } else {
            return false;
        }
    }

    /// Calculates the manhattan distance from the given position to [target]
    fn manhattan_distance(&self, position: (isize, isize)) -> usize {
        let dx = self.target.0.abs_diff(position.0);
        let dy = self.target.1.abs_diff(position.1);

        dx + dy
    }

    fn illustrate_path(&self, path: Vec<(isize, isize)>) {
        println!("{:?}", path);
        for y in 0..Y {
            for x in 0..X {
                if path.contains(&(x as isize, y as isize)) {
                    print!(
                        "{}",
                        ansi_term::Color::Blue
                            .bold()
                            .paint(format!("{}", self.risk[x][y]))
                    );
                } else if self.position == (x as isize, y as isize) {
                    print!(
                        "{}",
                        ansi_term::Color::Green
                            .bold()
                            .paint(format!("{}", self.risk[x][y]))
                    );
                } else if self.target == (x as isize, y as isize) {
                    print!(
                        "{}",
                        ansi_term::Color::Red
                            .bold()
                            .paint(format!("{}", self.risk[x][y]))
                    );
                } else {
                    print!("{}", self.risk[x][y]);
                }
            }
            print!("\n");
        }
    }
}

impl<const X: usize, const Y: usize> std::fmt::Display for CaveRisk<X, Y> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Y {
            for x in 0..X {
                if self.path.contains(&(x as isize, y as isize)) {
                    write!(
                        f,
                        "{}",
                        ansi_term::Style::new()
                            .bold()
                            .paint(format!("{}", self.risk[x][y]))
                    )?;
                } else {
                    write!(f, "{}", self.risk[x][y])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part_one<const X: usize, const Y: usize>(input: &str) -> isize {
    let mut cave: CaveRisk<X, Y> = CaveRisk::new(input);

    println!("{}\n", cave);

    let mut done = false;
    while !done {
        done = cave.step();
        // println!("{}\n", cave);
    }

    print!("{}", cave);

    cave.cost as isize
}

fn part_two(_input: &str) -> isize {
    let mut result = 0;
    result += 2;

    result
}

#[test]
fn example_part_one() {
    let result = part_one::<10, 10>(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 40);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 2);
}

#[test]
fn test_part_one() {
    let result = part_one::<100, 100>(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    // assert_eq!(42, result);
    more_asserts::assert_lt!(800, result);
}

#[test]
fn test_part_two() {
    // let result = part_two(OUR_INPUT.unwrap());
    // println!("part two: {}", result);
    // assert_eq!(42, result);
}
