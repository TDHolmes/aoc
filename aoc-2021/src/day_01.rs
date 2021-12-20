//! AOC Day xx
// use aoc_2021;

const DAY_01_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_01_part1.txt"));

fn part_one(input: &str) -> i32 {
    let mut count = 0;
    let mut previous_line = i32::MAX;
    for line in input.split_ascii_whitespace() {
        let line_parsed: i32 = str::parse(line).unwrap();
        print!("{:?} -> {}", line, line_parsed);

        if line_parsed > previous_line {
            println!(" inc!");
            count += 1;
        } else {
            println!(" dec.")
        }
        previous_line = line_parsed;
    }

    count
}

fn part_two(input: &str) -> i32 {
    let mut count = 0;
    let mut previous_sum = i32::MAX;
    let mut buf = vec![];
    for line in input.split_ascii_whitespace() {
        let line_parsed: i32 = str::parse(line).unwrap();
        buf.insert(0, line_parsed);
        if buf.len() < 3 {
            println!("");
            continue;
        }
        if buf.len() == 4 {
            buf.pop();
        }

        let sum: i32 = buf.iter().sum();

        if sum > previous_sum {
            println!("{} > {}? inc!", sum, previous_sum);
            count += 1;
        } else {
            println!("{} > {}? dec.", sum, previous_sum);
        }
        previous_sum = sum;
    }

    count
}

#[test]
fn example_part_one() {
    let example_result = part_one("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
    println!("result: {}", example_result);
    assert_eq!(7, example_result);
}

#[test]
fn example_part_two() {
    let example_result = part_two("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
    println!("result: {}", example_result);
    assert_eq!(5, example_result);
}

#[test]
fn test_part_one() {
    let input = DAY_01_INPUT.unwrap();
    let result = part_one(input);
    println!("output: {}", result);
    assert_eq!(1665, result);
}

#[test]
fn test_part_two() {
    let input = DAY_01_INPUT.unwrap();
    let result = part_two(input);
    println!("output: {}", result);
    assert_eq!(1702, result);
}
