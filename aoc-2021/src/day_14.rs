//! AOC Day xx
// use aoc_2021;

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

struct Polymerization {
    /// Our current polymer
    state: Vec<char>,
    /// Our pending state we build up during [`Self::step`].
    insertions: Vec<(usize, char)>,
    /// Our insertion rules
    rules: Vec<(char, char, char)>,
}

impl Polymerization {
    pub fn new(input: &str) -> Polymerization {
        let mut rules = vec![];
        let mut line_iter = input.split_terminator("\n");
        let state: Vec<char> = line_iter.next().unwrap().chars().collect();
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
            insertions: vec![],
            rules,
        }
    }

    pub fn step(&mut self) {
        let mut last = ' ';
        for (ind, c) in self.state.iter().enumerate() {
            for (l1, l2, r) in self.rules.iter() {
                if *l1 == last && *l2 == *c {
                    self.insertions.push((ind, *r));
                    // println!("Applying rule {}{}->{} (ind={})", l1, l2, r, ind);
                }
            }
            last = *c;
        }

        // insert the insertions while moving around the existing state values
        let mut insertions = 0;
        let mut right_end = self.state.len();
        self.state
            .resize(self.state.len() + self.insertions.len(), '-');
        let mut right_end_dest = self.state.len();
        for (ind, value) in self.insertions.drain(0..).rev() {
            let dest = right_end_dest - (right_end - ind);
            // println!(
            //     "Moving {}..{} to {}..{}",
            //     ind, right_end, dest, right_end_dest
            // );

            self.state.copy_within(ind..right_end, dest);
            *self.state.get_mut(ind).unwrap() = value;

            right_end_dest = dest;
            right_end = ind + 1;
            insertions += 1;
        }
        // self.print_state();
    }

    pub fn print_state(&self) {
        for c in self.state.iter() {
            print!("{}", c);
        }
        print!("\n");
    }

    pub fn compute(&self) -> usize {
        // let mut map = std::collections::HashMap::new();
        let mut sums: [usize; 26] = [0; 26];
        for (_i, c) in self.state.iter().enumerate() {
            sums[*c as usize - 'A' as usize] += 1;
        }

        let mut smallest = usize::MAX;
        let mut largest = usize::MIN;

        for val in sums.iter() {
            if *val > largest {
                largest = *val;
            }
            if *val < smallest && *val > 0 {
                smallest = *val;
            }
        }

        largest - smallest
    }
}

impl std::fmt::Display for Polymerization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Poly {{")?;
        write!(f, "\t")?;
        for c in self.state.iter() {
            write!(f, "{}", c)?;
        }
        write!(f, "\n\n")?;

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
    // let result = part_two(EXAMPLE_INPUT);
    // println!("example result: {}", result);
    // assert_eq!(result, 2188189693529);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(3118, result);
}

#[test]
fn test_part_two() {
    // let result = part_two(OUR_INPUT.unwrap());
    // println!("part two: {}", result);
    // assert_eq!(42, result);
}
