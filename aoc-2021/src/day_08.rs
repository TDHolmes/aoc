//! AOC Day xx
// use aoc_2021;

const ONE_CHARS: usize = 2;
const FOUR_CHARS: usize = 4;
const SEVEN_CHARS: usize = 3;
const EIGHT_CHARS: usize = 7;

const EXAMPLE_INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_08.txt"));

struct PossibleValues {
    a: Vec<char>,
    b: Vec<char>,
    c: Vec<char>,
    d: Vec<char>,
    e: Vec<char>,
    f: Vec<char>,
    g: Vec<char>,
}

impl PossibleValues {
    pub fn new() -> PossibleValues {
        let possible_values = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

        PossibleValues {
            a: possible_values.clone(),
            b: possible_values.clone(),
            c: possible_values.clone(),
            d: possible_values.clone(),
            e: possible_values.clone(),
            f: possible_values.clone(),
            g: possible_values.clone(),
        }
    }

    pub fn constrain(&mut self, actual_segments: &[char], mixed_segments: &[char]) {
        for actual in actual_segments.iter() {
            match actual {
                'a' => self.a.retain(|v| mixed_segments.contains(v)),
                'b' => self.b.retain(|v| mixed_segments.contains(v)),
                'c' => self.c.retain(|v| mixed_segments.contains(v)),
                'd' => self.d.retain(|v| mixed_segments.contains(v)),
                'e' => self.e.retain(|v| mixed_segments.contains(v)),
                'f' => self.f.retain(|v| mixed_segments.contains(v)),
                'g' => self.g.retain(|v| mixed_segments.contains(v)),
                _ => unreachable!(),
            }
        }
    }

    /// Given the input actual segment, return what we think it might translate to
    pub fn translate(&self, actual_segment: char) -> &Vec<char> {
        match actual_segment {
            'a' => &self.a,
            'b' => &self.b,
            'c' => &self.c,
            'd' => &self.d,
            'e' => &self.e,
            'f' => &self.f,
            'g' => &self.g,
            _ => unreachable!(),
        }
    }

    /// Given the input actual segment, return what we think it might translate to and allow
    /// the callee to modify that vec if it knows how to narrow it down
    pub fn translate_as_mut(&mut self, actual_segment: char) -> &mut Vec<char> {
        match actual_segment {
            'a' => &mut self.a,
            'b' => &mut self.b,
            'c' => &mut self.c,
            'd' => &mut self.d,
            'e' => &mut self.e,
            'f' => &mut self.f,
            'g' => &mut self.g,
            _ => unreachable!(),
        }
    }
}

impl core::fmt::Display for PossibleValues {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}: {:?}", 'a', self.a)?;
        writeln!(f, "{}: {:?}", 'b', self.b)?;
        writeln!(f, "{}: {:?}", 'c', self.c)?;
        writeln!(f, "{}: {:?}", 'd', self.d)?;
        writeln!(f, "{}: {:?}", 'e', self.e)?;
        writeln!(f, "{}: {:?}", 'f', self.f)?;
        writeln!(f, "{}: {:?}", 'g', self.g)?;
        Ok(())
    }
}

struct ScrambledDisplay {
    scrambled_to_real: std::collections::HashMap<char, char>,
}

impl ScrambledDisplay {
    pub fn new(a: char, b: char, c: char, d: char, e: char, f: char, g: char) -> Self {
        let mut scrambled_to_real = std::collections::HashMap::new();

        assert!(scrambled_to_real.insert(a, 'a').is_none());
        assert!(scrambled_to_real.insert(b, 'b').is_none());
        assert!(scrambled_to_real.insert(c, 'c').is_none());
        assert!(scrambled_to_real.insert(d, 'd').is_none());
        assert!(scrambled_to_real.insert(e, 'e').is_none());
        assert!(scrambled_to_real.insert(f, 'f').is_none());
        assert!(scrambled_to_real.insert(g, 'g').is_none());

        Self { scrambled_to_real }
    }

    pub fn to_display(&self, scrambled_input: &str) -> SevenSegmentDisplay {
        let mut display = SevenSegmentDisplay::new(false, false, false, false, false, false, false);
        for char in scrambled_input.chars() {
            display.set_segment(self.scrambled_to_real[&char], true);
        }

        display
    }
}

#[derive(Debug)]
struct SevenSegmentDisplay {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl SevenSegmentDisplay {
    pub fn new(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
        }
    }

    pub fn set_segment(&mut self, segment: char, value: bool) {
        match segment {
            'a' => self.a = value,
            'b' => self.b = value,
            'c' => self.c = value,
            'd' => self.d = value,
            'e' => self.e = value,
            'f' => self.f = value,
            'g' => self.g = value,
            _ => unreachable!(),
        }
    }

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Self {
                a: true,
                b: true,
                c: true,
                d: false,
                e: true,
                f: true,
                g: true,
            } => Some(0),
            Self {
                a: false,
                b: false,
                c: true,
                d: false,
                e: false,
                f: true,
                g: false,
            } => Some(1),
            Self {
                a: true,
                b: false,
                c: true,
                d: true,
                e: true,
                f: false,
                g: true,
            } => Some(2),
            Self {
                a: true,
                b: false,
                c: true,
                d: true,
                e: false,
                f: true,
                g: true,
            } => Some(3),
            Self {
                a: false,
                b: true,
                c: true,
                d: true,
                e: false,
                f: true,
                g: false,
            } => Some(4),
            Self {
                a: true,
                b: true,
                c: false,
                d: true,
                e: false,
                f: true,
                g: true,
            } => Some(5),
            Self {
                a: true,
                b: true,
                c: false,
                d: true,
                e: true,
                f: true,
                g: true,
            } => Some(6),
            Self {
                a: true,
                b: false,
                c: true,
                d: false,
                e: false,
                f: true,
                g: false,
            } => Some(7),
            Self {
                a: true,
                b: true,
                c: true,
                d: true,
                e: true,
                f: true,
                g: true,
            } => Some(8),
            Self {
                a: true,
                b: true,
                c: true,
                d: true,
                e: false,
                f: true,
                g: true,
            } => Some(9),
            _ => None,
        }
    }
}

fn parse_out_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let mut split_or = line.split(" | ");
    let ten_signals: Vec<&str> = split_or
        .next()
        .expect("invalid 10 unique signal input")
        .split(" ")
        .map(|v| v.trim())
        .collect();
    let numbers: Vec<&str> = split_or
        .next()
        .expect("invalid final output")
        .split(" ")
        .map(|v| v.trim())
        .collect();

    (ten_signals, numbers)
}

fn part_one(input: &str) -> isize {
    let mut num_interesting_chars = 0;
    for line in input.split_terminator("\n") {
        let (_ten_signals, numbers) = parse_out_line(line);

        for num in numbers {
            match num.len() {
                ONE_CHARS | FOUR_CHARS | SEVEN_CHARS | EIGHT_CHARS => num_interesting_chars += 1,
                _ => (),
            }
        }
    }

    num_interesting_chars
}

fn part_two(input: &str) -> isize {
    let mut four_char_value_sum = 0;
    for line in input.split_terminator("\n") {
        let mut true_match_found = false;
        let mut decoder = PossibleValues::new();
        println!("Parsing {:?}", line);
        let (ten_signals, numbers) = parse_out_line(line);
        for num in ten_signals.iter() {
            let mixed_segments: Vec<char> = num.chars().collect();
            match num.len() {
                ONE_CHARS => decoder.constrain(&['c', 'f'], &mixed_segments),
                FOUR_CHARS => decoder.constrain(&['b', 'c', 'd', 'f'], &mixed_segments),
                SEVEN_CHARS => decoder.constrain(&['a', 'c', 'f'], &mixed_segments),
                EIGHT_CHARS => (), // doesn't eliminate any characters
                _ => (),
            }
        }
        // We know that seven is just one with segment 'a' added, so we can fully constrain that segment
        let segment_c = decoder.translate('c').clone();
        decoder
            .translate_as_mut('a')
            .retain(|v| !segment_c.contains(v));

        // now brute force all possible combinations until we get all valid numbers out
        // forgive me for this
        let mut container: Vec<char> = Vec::with_capacity(7);
        'brute_force: for a in decoder.translate('a') {
            for b in decoder.translate('b') {
                for c in decoder.translate('c') {
                    for d in decoder.translate('d') {
                        for e in decoder.translate('e') {
                            for f in decoder.translate('f') {
                                for g in decoder.translate('g') {
                                    // assert that we have a unique display (no repeat segments)
                                    container.clear();
                                    container.extend_from_slice(&[*a, *b, *c, *d, *e, *f, *g]);
                                    container.sort();
                                    container.dedup();
                                    if container.len() < 7 {
                                        continue;
                                    }

                                    // construct a display and check if all of the numbers match what we expect
                                    let scrambled =
                                        ScrambledDisplay::new(*a, *b, *c, *d, *e, *f, *g);
                                    let mut is_match: bool = true;

                                    for input in ten_signals.iter() {
                                        if scrambled.to_display(*input).as_u8().is_none() {
                                            is_match = false;
                                            break;
                                        }
                                    }

                                    if is_match {
                                        let mut our_sum: isize = 0;
                                        for (ind, num) in numbers.iter().enumerate() {
                                            print!("{}=", *num);
                                            let disp = scrambled.to_display(*num);
                                            if let Some(digit) = disp.as_u8() {
                                                println!("{}", digit);
                                                our_sum += isize::from(digit)
                                                    * (10_isize.pow((3 - ind) as u32));
                                            } else {
                                                // failed to parse properly
                                                is_match = false;
                                                break;
                                            }
                                        }

                                        if is_match {
                                            println!("true match: {}", our_sum);
                                            true_match_found = true;
                                            // if it's still a match, excellent!
                                            four_char_value_sum += our_sum;
                                            break 'brute_force;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if !true_match_found {
            panic!("Failed to find a true decoding");
        }
    }

    four_char_value_sum
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 26);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 61_229);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(352, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(936_117, result);
}
