//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_04.txt"));

lazy_static::lazy_static! {
    static ref REGEX: regex::Regex = regex::Regex::new(r"(([\d]+)[\s]*){25}").expect("invalid regex");
}

struct BingoBoard<const ROWS: usize, const COLS: usize> {
    /// the numbers on our board.
    nums: [[u8; ROWS]; COLS],
    /// whether or not we've seen the given number from `nums`.
    bingo: [[bool; ROWS]; COLS],
    /// the summation of our bingos in the given row. If we reach `COLS`, we have a bingo.
    row_sum: [u8; ROWS],
    /// the summation of our bingos in the given column. If we reach `ROWS`, we have a bingo.
    col_sum: [u8; COLS],
}

impl<const ROWS: usize, const COLS: usize> BingoBoard<ROWS, COLS> {
    /// Creates a new bingo board from a string representation of the board.
    ///
    ///
    /// ```
    /// let bingo_board = BingoBoard::<5, 5>::new_from_string("14 21 17 24  4
    /// 10 16 15  9 19
    /// 18  8 23 26 20
    /// 22 11 13  6  5
    /// 2  0 12  3  7");
    /// ```
    pub fn new_from_string(bingo_board_string: &str) -> BingoBoard<ROWS, COLS> {
        let mut board = BingoBoard {
            nums: [[0; ROWS]; COLS],
            bingo: [[false; ROWS]; COLS],
            row_sum: [0; ROWS],
            col_sum: [0; COLS],
        };

        for (row, line) in bingo_board_string.split("\n").enumerate() {
            for (col, num) in line.split_ascii_whitespace().enumerate() {
                board.nums[col][row] = num.parse().expect("invalid number");
            }
        }

        board
    }

    /// Checks if this board has a bingo.
    fn has_bingo(&self) -> bool {
        for sum in self.row_sum {
            if usize::from(sum) == COLS {
                return true;
            }
        }

        for sum in self.col_sum {
            if usize::from(sum) == ROWS {
                return true;
            }
        }

        false
    }

    /// Takes in a new number and checks it against our board. Returns `true` if we now have a bingo.
    pub fn ingest_number(&mut self, number: u8) -> bool {
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.nums[col][row] == number {
                    self.bingo[col][row] = true;
                    self.row_sum[row] += 1;
                    self.col_sum[col] += 1;

                    if usize::from(self.row_sum[row]) == COLS
                        || usize::from(self.col_sum[col]) == ROWS
                    {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }

        false
    }

    /// If this board has a bingo, compute the bingo score (summation of the numbers involved in the bingo).
    pub fn compute_score(&self) -> usize {
        let mut score: usize = 0;
        for row in 0..ROWS {
            for col in 0..COLS {
                if !self.bingo[col][row] {
                    score += self.nums[col][row] as usize;
                }
            }
        }

        score
    }
}

impl<const ROWS: usize, const COLS: usize> core::fmt::Display for BingoBoard<ROWS, COLS> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for row in 0..ROWS {
            for col in 0..COLS {
                let (pre, post) = if self.bingo[col][row] {
                    ("\x1b[1;4m", "\x1b[0m")
                } else {
                    ("", "")
                };
                write!(f, "{}{:3}{}", pre, self.nums[col][row], post)?
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn part_one(input: &str) -> isize {
    let first_line_ind = input.find("\n").expect("invalid input format");
    let first_line = &input[..first_line_ind];
    let boards_input = &input[first_line_ind..];
    let moves: Vec<u8> = first_line
        .split(",")
        .map(|num| num.parse().expect("invalid input moves"))
        .collect();

    let mut boards: Vec<BingoBoard<5, 5>> = Vec::new();
    for cap in REGEX.captures_iter(boards_input) {
        let board = BingoBoard::new_from_string(&cap[0].trim());
        boards.push(board);
    }

    let mut bingo_score: isize = 0;
    for bingo_num in &moves {
        let mut has_bingo = false;
        for board in &mut boards {
            let bingo = board.ingest_number(*bingo_num);
            println!("{}", board);
            if bingo {
                bingo_score = board.compute_score() as isize * *bingo_num as isize;
                has_bingo = true;
                break;
            }
        }

        if has_bingo {
            println!("bingo!!");
            break;
        }
    }

    bingo_score
}

fn part_two(input: &str) -> isize {
    let first_line_ind = input.find("\n").expect("invalid input format");
    let first_line = &input[..first_line_ind];
    let boards_input = &input[first_line_ind..];
    let moves: Vec<u8> = first_line
        .split(",")
        .map(|num| num.parse().expect("invalid input moves"))
        .collect();

    let mut boards: Vec<BingoBoard<5, 5>> = Vec::new();
    for cap in REGEX.captures_iter(boards_input) {
        let board = BingoBoard::new_from_string(&cap[0].trim());
        boards.push(board);
    }

    let mut bingo_score: isize = 0;
    for bingo_num in &moves {
        let mut has_bingo = false;
        for board in &mut boards {
            let bingo = board.ingest_number(*bingo_num);
            println!("{}", board);
            if bingo {
                bingo_score = board.compute_score() as isize * *bingo_num as isize;
                has_bingo = true;
            }
        }

        if has_bingo && boards.len() == 1 {
            println!("last board has finally won");
            break;
        }
        boards.retain(|b| !b.has_bingo());
    }

    bingo_score
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 4512);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 1924);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(55_770, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(2980, result);
}
