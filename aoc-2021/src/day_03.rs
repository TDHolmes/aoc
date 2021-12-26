//! AOC Day xx
// use aoc_2021;

/// computes gamma and epsilon rates
fn compute_gamma_epsilon<'a>(input: &mut impl Iterator<Item = &'a &'a str>) -> (usize, usize) {
    const MAX_BITS: usize = 16;

    let mut num_inputs: usize = 0;
    let mut num_bits = 0;
    let mut bits: [usize; MAX_BITS] = [0; MAX_BITS];

    for line in input {
        if num_bits == 0 {
            num_bits = line.len();
        }

        for (ind, char) in line.chars().enumerate() {
            match char {
                '1' => bits[num_bits - ind - 1] += 1,
                '0' => (),
                _ => panic!("invalid bit {}", char),
            }
        }

        num_inputs += 1;
    }
    assert!(num_bits <= MAX_BITS);

    // compute gamma
    let mut gamma_rate = 0;
    for bit in 0..num_bits {
        // check for most common bit, rounding up
        if bits[bit] >= (num_inputs + 1) / 2 {
            gamma_rate |= 1 << bit;
        }
    }

    let mask = (1 << num_bits) - 1;
    let epsilon_rate = (!gamma_rate) & mask;

    (gamma_rate, epsilon_rate)
}

fn str_bits_to_num(line: &str) -> usize {
    let mut num = 0;
    let num_bits = line.len();
    for (ind, char) in line.chars().enumerate() {
        match char {
            '1' => num |= 1 << (num_bits - ind - 1),
            '0' => (),
            _ => panic!("invalid bit {}", char),
        }
    }

    num
}

const EXAMPLE_INPUT: &str =
    "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";
const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_03.txt"));

fn part_one(input: &'static str) -> usize {
    let input: Vec<&'static str> = input.split_terminator("\n").collect();
    let (gamma, epsilon) = compute_gamma_epsilon(&mut input.iter());

    gamma * epsilon
}

fn part_two(input: &'static str) -> usize {
    let mut oxy_input: Vec<&'static str> = input.split_terminator("\n").collect();
    let mut co2_input: Vec<&'static str> = input.split_terminator("\n").collect();
    let num_bits = oxy_input[0].len();

    for bit in (0..num_bits).rev() {
        // for oxy in &oxy_input {
        //     print!(" {}", oxy)
        // }
        // println!("");

        let (oxy_gamma, _) = compute_gamma_epsilon(&mut oxy_input.iter());
        oxy_input.retain(|input| {
            let char = if oxy_gamma & (1 << bit) != 0 {
                '1'
            } else {
                '0'
            };

            input
                .chars()
                .nth(num_bits - bit - 1)
                .expect("invalid input")
                == char
        });

        if oxy_input.len() == 1 {
            break;
        }
    }

    for bit in (0..num_bits).rev() {
        // for oxy in &co2_input {
        //     print!(" {}", oxy)
        // }
        // println!("");
        let (_co2_gamma, co2_epsilon) = compute_gamma_epsilon(&mut co2_input.iter());
        let character = if co2_epsilon & (1 << bit) != 0 {
            '1'
        } else {
            '0'
        };
        println!("bit {} {:05b} - expect {}", bit, co2_epsilon, character);

        co2_input.retain(|input| {
            input
                .chars()
                .nth(num_bits - bit - 1)
                .expect("invalid input")
                == character
        });

        if co2_input.len() == 1 {
            break;
        }
    }

    assert_eq!(1, oxy_input.len());
    assert_eq!(1, co2_input.len());

    let oxy = str_bits_to_num(oxy_input[0]);
    let co2 = str_bits_to_num(co2_input[0]);
    println!("oxy: {0:b} {0}", oxy);
    println!("co2: {0:b} {0}", co2);

    oxy * co2
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 198);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 230);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(2261546, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(6775520, result);
}
