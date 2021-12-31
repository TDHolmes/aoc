//! AOC Day xx
// use aoc_2021;

use std::collections::VecDeque;

const EXAMPLE_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_10.txt"));

fn unexpected_deliminator_to_score(delim: char) -> usize {
    match delim {
        ')' => 3,     // | '('
        ']' => 57,    // | '['
        '}' => 1197,  // | '{'
        '>' => 25137, // | '<'
        _ => unreachable!(),
    }
}

fn autocomplete_deliminator_to_score(delim: char) -> usize {
    match delim {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn complementary_deliminator(delim: char) -> char {
    match delim {
        ')' => '(',
        '(' => ')',
        ']' => '[',
        '[' => ']',
        '}' => '{',
        '{' => '}',
        '>' => '<',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn parse_line_for_syntax(line: &str) -> usize {
    let mut chunks: VecDeque<char> = VecDeque::new();
    for character in line.chars() {
        match character {
            ')' | ']' | '}' | '>' => {
                let found = chunks.pop_back().expect("unbalanced chunk");
                let expected = complementary_deliminator(character);
                if found != expected {
                    println!("Expected {:?}, found {:?}", expected, character);
                    return unexpected_deliminator_to_score(character);
                }
            }
            '(' | '[' | '{' | '<' => chunks.push_back(character),
            _ => unreachable!(),
        }
    }

    if chunks.len() != 0 {
        println!("incomplete line");
    }

    0
}

fn parse_line_for_autocomplete(line: &str) -> usize {
    let mut chunks: VecDeque<char> = VecDeque::new();
    for character in line.chars() {
        match character {
            ')' | ']' | '}' | '>' => {
                let found = chunks.pop_back().expect("unbalanced chunk");
                let expected = complementary_deliminator(character);
                if found != expected {
                    println!("Expected {:?}, found {:?}", expected, character);
                    return 0;
                }
            }
            '(' | '[' | '{' | '<' => chunks.push_back(character),
            _ => unreachable!(),
        }
    }

    let mut score = 0;
    if chunks.len() != 0 {
        println!("incomplete line");
        while let Some(found) = chunks.pop_back() {
            match found {
                '(' | '[' | '{' | '<' => {
                    let closer = complementary_deliminator(found);
                    println!("adding {:?}", closer);
                    score *= 5;
                    score += autocomplete_deliminator_to_score(closer);
                }
                _ => unreachable!(),
            }
        }
    }

    score
}

fn part_one(input: &str) -> usize {
    let mut result = 0;
    for line in input.split_terminator('\n') {
        let cost = parse_line_for_syntax(line);
        result += cost;
    }

    result
}

fn part_two(input: &str) -> usize {
    let mut costs = vec![];
    for line in input.split_terminator('\n') {
        let cost = parse_line_for_autocomplete(line);
        if cost != 0 {
            costs.push(cost);
        }
    }
    costs.sort();

    costs[costs.len() / 2]
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 26397);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 288_957);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(319_233, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(1_118_976_874, result);
}
