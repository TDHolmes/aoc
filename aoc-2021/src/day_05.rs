//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_05.txt"));

#[derive(Debug)]
struct Line {
    start: (isize, isize),
    end: (isize, isize),
}

impl Line {
    pub fn new(start: (isize, isize), end: (isize, isize)) -> Option<Self> {
        println!("Line::new({:?}, {:?})", start, end);
        let x_eq: bool = start.0 == end.0;
        let y_eq: bool = start.1 == end.1;
        let diag = (end.0 - start.0).abs() == (end.1 - start.1).abs();
        if !x_eq && !y_eq && !diag {
            return None;
        }

        let swap = if x_eq && start.1 > end.1 {
            true
        } else if y_eq && start.0 > end.0 {
            true
        } else {
            false
        };

        if !swap {
            Some(Self { start, end })
        } else {
            Some(Self {
                start: end,
                end: start,
            })
        }
    }

    /// Creates a new line from an input string like `0,9 -> 5,9`.
    pub fn from_string(string: &str) -> Option<Self> {
        let mut line_split = string.split(" -> ");
        let start_str = line_split.next()?;
        let end_str = line_split.next()?;

        let start: Vec<isize> = start_str
            .split(",")
            .map(|num| num.parse().expect("invalid num"))
            .collect();
        let end: Vec<isize> = end_str
            .split(",")
            .map(|num| num.parse().expect("invalid num"))
            .collect();

        assert_eq!(start.len(), 2);
        assert_eq!(end.len(), 2);

        Line::new((start[0], start[1]), (end[0], end[1]))
    }

    pub fn iter<'a>(&'a self, allow_diagonal: bool) -> LineIter<'a> {
        let mut diag_delta = (0, 0);
        let iter_method = if self.start.1 == self.end.1 {
            IterMethod::ByCols
        } else if self.start.0 == self.end.0 {
            IterMethod::ByRows
        } else if (self.end.0 - self.start.0).abs() == (self.end.1 - self.start.1).abs() {
            diag_delta = (
                (self.end.0 - self.start.0) / (self.end.0 - self.start.0).abs(),
                (self.end.1 - self.start.1) / (self.end.1 - self.start.1).abs(),
            );
            IterMethod::ByDiagonal
        } else {
            panic!("invalid line for iteration");
        };
        LineIter {
            line: self,
            /// if the y coords are the same, we're iterating along the x axis
            method: iter_method,
            allow_diagonal,
            delta: 0,
            diag_delta,
        }
    }
}

enum IterMethod {
    ByRows,
    ByCols,
    ByDiagonal,
}

struct LineIter<'a> {
    line: &'a Line,
    method: IterMethod,
    allow_diagonal: bool,
    delta: isize,
    diag_delta: (isize, isize),
}

impl Iterator for LineIter<'_> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.method {
            IterMethod::ByCols => {
                if self.line.start.0 + self.delta > self.line.end.0 {
                    None
                } else {
                    Some((self.line.start.0 + self.delta, self.line.start.1))
                }
            }
            IterMethod::ByRows => {
                if self.line.start.1 + self.delta > self.line.end.1 {
                    None
                } else {
                    Some((self.line.start.0, self.line.start.1 + self.delta))
                }
            }
            IterMethod::ByDiagonal => {
                if !self.allow_diagonal || self.delta > (self.line.end.0 - self.line.start.0).abs()
                {
                    None
                } else {
                    Some((
                        self.line.start.0 + self.diag_delta.0 * self.delta,
                        self.line.start.1 + self.diag_delta.1 * self.delta,
                    ))
                }
            }
        };

        self.delta += 1;

        res
    }
}

struct ThermalVentsField<const ROWS: usize, const COLS: usize> {
    allow_diagonal: bool,
    field: [[u8; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> ThermalVentsField<ROWS, COLS> {
    pub const fn new(allow_diagonal: bool) -> Self {
        Self {
            allow_diagonal,
            field: [[0; COLS]; ROWS],
        }
    }

    pub fn input_line(&mut self, line: Line) {
        for (x, y) in line.iter(self.allow_diagonal) {
            self.field[y as usize][x as usize] += 1;
        }
    }

    pub fn check_intersections(&self, threshold: u8) -> isize {
        let mut count = 0;
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.field[col][row] >= threshold {
                    count += 1;
                }
            }
        }

        count
    }
}

impl<const ROWS: usize, const COLS: usize> core::fmt::Display for ThermalVentsField<ROWS, COLS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for row in 0..ROWS {
            for col in 0..COLS {
                let (pre, post) = if self.field[col][row] != 0 {
                    ("\x1b[1;4m", "\x1b[0m")
                } else {
                    ("", "")
                };
                write!(f, "{}{:2}{}", pre, self.field[col][row], post)?
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn part_one<const ROWS: usize, const COLS: usize>(input: &str) -> isize {
    let mut field = ThermalVentsField::<ROWS, COLS>::new(false);
    let mut lines: Vec<Line> = Vec::<Line>::new();
    for line in input.split_terminator("\n") {
        if let Some(line) = Line::from_string(line) {
            lines.push(line);
        }
    }
    for line in lines.into_iter() {
        field.input_line(line);
    }
    println!("{}", field);

    field.check_intersections(2)
}

fn part_two<const ROWS: usize, const COLS: usize>(input: &str) -> isize {
    let mut field = ThermalVentsField::<ROWS, COLS>::new(true);
    let mut lines: Vec<Line> = Vec::<Line>::new();
    for line in input.split_terminator("\n") {
        if let Some(line) = Line::from_string(line) {
            lines.push(line);
        }
    }
    for line in lines.into_iter() {
        field.input_line(line);
    }
    println!("{}", field);

    field.check_intersections(2)
}

#[test]
fn example_part_one() {
    let result = part_one::<10, 10>(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 5);
}

#[test]
fn example_part_two() {
    let result = part_two::<10, 10>(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 12);
}

#[test]
fn test_part_one() {
    let result = part_one::<1_000, 1_000>(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(5169, result);
}

#[test]
fn test_part_two() {
    let result = part_two::<1_000, 1_000>(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(22_083, result);
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn line_from_str() {
        let line = Line::from_string("0,9 -> 5,9").unwrap();
        assert_eq!(line.start, (0, 9));
        assert_eq!(line.end, (5, 9));
    }

    #[test]
    fn line_iter_x() {
        let line = Line::from_string("0,9 -> 5,9").unwrap();
        let mut line_iter = line.iter(false);
        assert_eq!(Some((0, 9)), line_iter.next());
        assert_eq!(Some((1, 9)), line_iter.next());
        assert_eq!(Some((2, 9)), line_iter.next());
        assert_eq!(Some((3, 9)), line_iter.next());
        assert_eq!(Some((4, 9)), line_iter.next());
        assert_eq!(Some((5, 9)), line_iter.next());
        assert_eq!(None, line_iter.next());
    }

    #[test]
    fn line_iter_y() {
        let line = Line::from_string("9,0 -> 9,5").unwrap();
        let mut line_iter = line.iter(false);
        assert_eq!(Some((9, 0)), line_iter.next());
        assert_eq!(Some((9, 1)), line_iter.next());
        assert_eq!(Some((9, 2)), line_iter.next());
        assert_eq!(Some((9, 3)), line_iter.next());
        assert_eq!(Some((9, 4)), line_iter.next());
        assert_eq!(Some((9, 5)), line_iter.next());
        assert_eq!(None, line_iter.next());
    }

    #[test]
    fn line_iter_x_rev() {
        let line = Line::from_string("5,9 -> 0,9").unwrap();
        println!("line: {:?}", &line);
        let mut line_iter = line.iter(false);
        assert_eq!(Some((0, 9)), line_iter.next());
        assert_eq!(Some((1, 9)), line_iter.next());
        assert_eq!(Some((2, 9)), line_iter.next());
        assert_eq!(Some((3, 9)), line_iter.next());
        assert_eq!(Some((4, 9)), line_iter.next());
        assert_eq!(Some((5, 9)), line_iter.next());
        assert_eq!(None, line_iter.next());
    }

    #[test]
    fn line_iter_diag() {
        let line = Line::from_string("4,4 -> 7,7").unwrap();
        println!("line: {:?}", &line);
        let mut line_iter = line.iter(true);
        assert_eq!(Some((4, 4)), line_iter.next());
        assert_eq!(Some((5, 5)), line_iter.next());
        assert_eq!(Some((6, 6)), line_iter.next());
        assert_eq!(Some((7, 7)), line_iter.next());
        assert_eq!(None, line_iter.next());
    }

    #[test]
    fn line_iter_diag_rev() {
        let line = Line::from_string("7,7 -> 4,4").unwrap();
        println!("line: {:?}", &line);
        let mut line_iter = line.iter(true);
        assert_eq!(Some((7, 7)), line_iter.next());
        assert_eq!(Some((6, 6)), line_iter.next());
        assert_eq!(Some((5, 5)), line_iter.next());
        assert_eq!(Some((4, 4)), line_iter.next());
        assert_eq!(None, line_iter.next());
    }
}
