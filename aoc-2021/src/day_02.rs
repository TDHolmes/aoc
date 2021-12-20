//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";
const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_02.txt"));

fn part_one(input: &str) -> isize {
    let mut horizontal: isize = 0;
    let mut depth: isize = 0;

    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }

        let mut split = line.split(" ");
        let dir = split.next().unwrap_or_else(|| panic!("{:?} invalid", line));
        let mag = split.next().unwrap_or_else(|| panic!("{:?} invalid", line));

        let mag: isize = mag
            .parse()
            .unwrap_or_else(|_| panic!("{:?} isn't a number", mag));

        let (horizontal_delta, depth_delta) = match dir {
            "forward" => (mag, 0),
            "up" => (0, -mag),
            "down" => (0, mag),
            _ => panic!("unexpected input {:?}", dir),
        };

        horizontal += horizontal_delta;
        depth += depth_delta;
    }

    horizontal * depth
}

fn part_two(input: &str) -> isize {
    let mut horizontal: isize = 0;
    let mut depth: isize = 0;
    let mut aim = 0;

    for line in input.split("\n") {
        if line.len() == 0 {
            continue;
        }

        let mut split = line.split(" ");
        let dir = split.next().unwrap_or_else(|| panic!("{:?} invalid", line));
        let mag = split.next().unwrap_or_else(|| panic!("{:?} invalid", line));

        let mag: isize = mag
            .parse()
            .unwrap_or_else(|_| panic!("{:?} isn't a number", mag));

        if let Some((horizontal_delta, depth_delta)) = match dir {
            "forward" => Some((mag, mag * aim)),
            "up" => {
                aim -= mag;
                None
            }
            "down" => {
                aim += mag;
                None
            }
            _ => panic!("unexpected input {:?}", dir),
        } {
            horizontal += horizontal_delta;
            depth += depth_delta;
        }
    }

    horizontal * depth
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    assert_eq!(result, 150);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    assert_eq!(result, 2);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("Part one: {}", result);
    assert_eq!(result, 1855814);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("Part two: {}", result);
    assert_eq!(result, 1845455714);
}
