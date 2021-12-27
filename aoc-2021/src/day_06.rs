//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "3,4,3,1,2";
const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_06.txt"));

#[derive(Debug)]
struct Fish {
    pub(crate) num: usize,
    pub(crate) cycle: u8,
}

impl Fish {
    pub const fn new(num: usize, cycle: u8) -> Self {
        Self { num, cycle }
    }
}

fn simulate(input: &str, num_days: usize, verbose: bool) -> isize {
    let mut state: Vec<Fish> = input
        .split(",")
        .map(|v| Fish::new(1, v.parse().expect("invalid state")))
        .collect();
    // let mut new_fish: Vec<u8> = Vec::new();
    for day in 0..num_days {
        let mut new_fish_count = 0;
        for fish in state.iter_mut() {
            if fish.cycle == 0 {
                new_fish_count += fish.num;
                fish.cycle = 6;
            } else {
                fish.cycle -= 1;
            }
        }

        // new_fish.resize(new_fish_count, 8);
        state.push(Fish::new(new_fish_count, 8));

        if verbose {
            println!("{:2}: {:?}", day, state);
        } else if day % 16 == 0 {
            println!("day {:2}", day);
        }
    }

    let mut num_fish: isize = 0;
    for fish in state {
        num_fish += fish.num as isize;
    }

    num_fish
}

fn part_one(input: &str) -> isize {
    const NUM_DAYS: usize = 80;

    simulate(input, NUM_DAYS, false)
}

fn part_two(input: &str) -> isize {
    const NUM_DAYS: usize = 256;

    simulate(input, NUM_DAYS, false)
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 5934);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 26_984_457_539);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(391_671, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(1_754_000_560_399, result);
}
