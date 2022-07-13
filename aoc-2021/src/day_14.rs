//! AOC Day xx
// use aoc_2021;

use std::collections::HashMap;

const EXAMPLE_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_14.txt"));

#[derive(Copy, Clone)]
struct PolyPair {
    /// The polymer pair
    pub pair: [char; 2],
    /// The number of instances of this polymerization pair
    pub quantity: usize,
}

impl PolyPair {
    pub fn new(pair: [char; 2], quantity: usize) -> PolyPair {
        PolyPair { pair, quantity }
    }

    pub fn insert(self, insertion: char) -> (PolyPair, PolyPair) {
        let first = PolyPair {
            pair: [self.pair[0], insertion],
            quantity: self.quantity,
        };
        let second = PolyPair {
            pair: [insertion, self.pair[1]],
            quantity: self.quantity,
        };

        // println!("{} -> {}, {}", self, first, second);

        (first, second)
    }
}

impl std::fmt::Display for PolyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{'{}{}', {}}}",
            self.pair[0], self.pair[1], self.quantity
        )?;
        Ok(())
    }
}

struct Polymerization {
    /// Our current state
    state: Vec<PolyPair>,
    /// Our pending
    pending: Vec<PolyPair>,
    /// Our insertion rules
    rules: Vec<(char, char, char)>,
}

impl Polymerization {
    pub fn new(input: &str) -> Polymerization {
        let mut rules = vec![];
        let mut line_iter = input.split_terminator("\n");
        let mut previous = ' ';
        let mut state: Vec<PolyPair> = vec![];
        for char in line_iter.next().unwrap().chars() {
            state.push(PolyPair::new([previous, char], 1));
            previous = char;
        }
        state.push(PolyPair::new([previous, ' '], 1));

        line_iter.next().unwrap(); // empty newline

        for line in line_iter {
            let mut split = line.split(" -> ");
            let left = split.next().expect("invalid rule");
            let right = split.next().expect("invalid rule");

            let mut left_chars = left.chars();
            rules.push((
                left_chars.next().unwrap(),
                left_chars.next().unwrap(),
                right.chars().next().unwrap(),
            ));
        }

        Polymerization {
            state,
            pending: vec![],
            rules,
        }
    }

    pub fn step(&mut self) {
        for pair in self.state.drain(0..) {
            let mut match_found = false;
            for (l1, l2, r) in self.rules.iter() {
                if pair.pair[0] == *l1 && pair.pair[1] == *l2 {
                    let (p1, p2) = pair.insert(*r);
                    Self::insert_to_pending(&mut self.pending, p1);
                    Self::insert_to_pending(&mut self.pending, p2);
                    match_found = true;
                    break;
                }
            }

            if !match_found {
                Self::insert_to_pending(&mut self.pending, pair);
            }
        }

        std::mem::swap(&mut self.state, &mut self.pending);
    }

    fn insert_to_pending(pending: &mut Vec<PolyPair>, pair: PolyPair) {
        // check if pair already exists in pending
        let mut found_match = false;
        for pending_pair in pending.iter_mut() {
            if pending_pair.pair == pair.pair {
                pending_pair.quantity += pair.quantity;
                found_match = true;
                break;
            }
        }

        if !found_match {
            pending.push(pair);
        }
    }

    #[allow(dead_code)]
    pub fn print_state(&self) {
        print!("\n");
    }

    pub fn compute(&self) -> usize {
        let mut count = HashMap::new();

        for pair in self.state.iter() {
            for c in pair.pair.iter() {
                if let Some(v) = count.get_mut(c) {
                    *v += pair.quantity;
                } else {
                    count.insert(*c, pair.quantity);
                }
            }
        }

        count.remove(&' '); // remove the dummy chars

        let mut min = usize::MAX;
        let mut max = usize::MIN;

        for (_k, v) in count.iter() {
            if *v < min {
                min = *v;
            }
            if *v > max {
                max = *v;
            }
        }

        max / 2 - min / 2
    }
}

impl std::fmt::Display for Polymerization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Poly {{")?;
        for c in self.state.iter() {
            writeln!(f, "\t{}", c)?;
        }
        write!(f, "\n")?;

        for (l1, l2, r) in self.rules.iter() {
            writeln!(f, "\t{}{} -> {}", l1, l2, r)?;
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}

fn part_one(input: &str) -> isize {
    let mut poly = Polymerization::new(input);
    println!("{}", poly);

    for _i in 0..10 {
        poly.step();
    }

    poly.compute() as isize
}

fn part_two(input: &str) -> isize {
    let mut poly = Polymerization::new(input);
    println!("{}", poly);

    for i in 0..40 {
        println!("step {}", i);
        poly.step();
    }

    poly.compute() as isize
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 1588);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 2188189693529);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(3118, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(4332887448171, result);
}
