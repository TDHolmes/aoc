//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_11.txt"));

#[derive(Debug)]
struct DumboOctopus<const ROWS: usize, const COLS: usize> {
    map: [[u8; ROWS]; COLS],
}

impl<const ROWS: usize, const COLS: usize> DumboOctopus<ROWS, COLS> {
    pub fn from_str(input: &str) -> Self {
        let mut map = Self {
            map: [[0xF_u8; ROWS]; COLS],
        };
        for (row, line) in input.split_terminator("\n").enumerate() {
            assert_eq!(COLS - 2, line.len());
            for (col, char) in line.chars().enumerate() {
                map.map[col + 1][row + 1] = char.to_digit(10).expect("invalid character") as u8;
            }
        }

        map
    }

    pub fn iter_indices(&self) -> DumboOctopusIndices<ROWS, COLS> {
        DumboOctopusIndices::<ROWS, COLS> { index: (0, 1) } // initialize x to zero for the first += 1
    }

    /// run a step of the simulation, returning the number of DumboOctopi that flash
    pub fn step(&mut self) -> usize {
        // first step, increment everyone by one
        for (col, row) in self.iter_indices() {
            self.map[col][row] += 1;
        }

        // second step, do {
        //    - look for flashers (== 9)
        //    - if flashed, set energy to 0
        //    - All adjacent Octopi to flashers increment by one if they're non-zero
        // } while (someone flashed)
        let mut total_flashed = 0;
        loop {
            let flashed = self.second_step();
            if flashed == 0 {
                break;
            } else {
                total_flashed += flashed;
            }
        }

        total_flashed
    }

    fn second_step(&mut self) -> usize {
        // first step, increment everyone by one
        let mut flashed = 0;
        for (col, row) in self.iter_indices() {
            if self.map[col][row] == 10 {
                // flash!
                self.map[col][row] = 0;
                flashed += 1;

                // increment surrounding octopi
                if (1..=9).contains(&self.map[col - 1][row - 1]) {
                    self.map[col - 1][row - 1] += 1;
                }
                if (1..=9).contains(&self.map[col - 1][row]) {
                    self.map[col - 1][row] += 1;
                }
                if (1..=9).contains(&self.map[col - 1][row + 1]) {
                    self.map[col - 1][row + 1] += 1;
                }
                if (1..=9).contains(&self.map[col][row - 1]) {
                    self.map[col][row - 1] += 1;
                }
                if (1..=9).contains(&self.map[col][row + 1]) {
                    self.map[col][row + 1] += 1;
                }
                if (1..=9).contains(&self.map[col + 1][row - 1]) {
                    self.map[col + 1][row - 1] += 1;
                }
                if (1..=9).contains(&self.map[col + 1][row]) {
                    self.map[col + 1][row] += 1;
                }
                if (1..=9).contains(&self.map[col + 1][row + 1]) {
                    self.map[col + 1][row + 1] += 1;
                }
            }
        }

        flashed
    }
}

struct DumboOctopusIndices<const ROWS: usize, const COLS: usize> {
    index: (usize, usize),
}

impl<const ROWS: usize, const COLS: usize> Iterator for DumboOctopusIndices<ROWS, COLS> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index.0 < COLS - 2 {
            self.index.0 += 1;
        } else {
            self.index.0 = 1;
            if self.index.1 < ROWS - 2 {
                self.index.1 += 1;
            } else {
                return None;
            }
        }

        Some(self.index)
    }
}

impl<const ROWS: usize, const COLS: usize> core::fmt::Display for DumboOctopus<ROWS, COLS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (col, row) in self.iter_indices() {
            let (pre, post) = if self.map[col][row] == 0 {
                ("\x1b[1;4m", "\x1b[0m")
            } else {
                ("", "")
            };
            write!(f, "{}{}{}", pre, self.map[col][row], post)?;
            if col == COLS - 2 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

fn part_one(input: &str) -> usize {
    const NUM_STEPS: usize = 100;
    // it's actually 10 by 10, but we have a padding row/col on all sides
    let mut map = DumboOctopus::<12, 12>::from_str(input);
    let mut total_flashed = 0;
    println!("initial:\n{}", map);
    for step in 0..NUM_STEPS {
        total_flashed += map.step();
        {
            println!("After step {}:", step);
            println!("{}", map)
        }
    }

    total_flashed
}

fn part_two(input: &str) -> isize {
    // it's actually 10 by 10, but we have a padding row/col on all sides
    let mut map = DumboOctopus::<12, 12>::from_str(input);

    println!("initial:\n{}", map);
    let mut step = 0;
    let synchronized_step = loop {
        let flashed = map.step();
        step += 1;

        println!("step {}: {} flashed", step, flashed);
        if flashed == 10 * 10 {
            println!("\tsynchronized!");
            break step;
        }
    };

    synchronized_step
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 1656);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 195);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(1644, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(229, result);
}
