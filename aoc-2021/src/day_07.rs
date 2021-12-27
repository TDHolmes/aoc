//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";
const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_07.txt"));

fn part_one(input: &str) -> isize {
    let mut crab_position: Vec<usize> = input
        .split(",")
        .map(|v| v.parse().expect("invalid position"))
        .collect();
    crab_position.sort();
    let median = crab_position[crab_position.len() / 2];
    let mut fuel_cost = isize::MAX;
    for med in (median - 2)..(median + 2) {
        println!("moving crabs to {}", median);
        let fuel: isize = crab_position
            .iter()
            .map(|pos| (*pos as isize - med as isize).abs())
            .sum();
        println!("fuel: {}", fuel);
        if fuel < fuel_cost {
            fuel_cost = fuel;
        }
    }

    fuel_cost
}

fn part_two(input: &str) -> isize {
    let crab_position: Vec<usize> = input
        .split(",")
        .map(|v| v.parse().expect("invalid position"))
        .collect();

    let crab_pos_sum: usize = crab_position.iter().sum();
    let average = (crab_pos_sum as f32 / crab_position.len() as f32).round() as isize;
    let mut fuel_cost = isize::MAX;

    for ave in (average - 2)..(average + 2) {
        println!("moving crabs to {}", average);
        let fuel: isize = crab_position
            .iter()
            .map(|pos| {
                let delta = (*pos as isize - ave as isize).abs();
                (delta * delta + delta) / 2 // see https://en.wikipedia.org/wiki/Binomial_coefficient
            })
            .sum();
        println!("fuel: {}", fuel);
        if fuel < fuel_cost {
            fuel_cost = fuel;
        }
    }

    fuel_cost
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 37);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 168);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(341558, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(93_214_037, result);
}
