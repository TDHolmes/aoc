//! --- Day 12: Rain Risk ---
//!
//! Your ferry made decent progress toward the island, but the storm came in
//! faster than anyone expected. The ferry needs to take evasive actions!
//!
//! Unfortunately, the ship's navigation computer seems to be malfunctioning;
//! rather than giving a route directly to safety, it produced extremely circuitous
//! instructions. When the captain uses the PA system to ask if anyone can help,
//! you quickly volunteer.
//!
//! The navigation instructions (your puzzle input) consists of a sequence of
//! single-character actions paired with integer input values. After staring at
//! them for a few minutes, you work out what they probably mean:
//!   - Action N means to move north by the given value.
//!   - Action S means to move south by the given value.
//!   - Action E means to move east by the given value.
//!   - Action W means to move west by the given value.
//!   - Action L means to turn left the given number of degrees.
//!   - Action R means to turn right the given number of degrees.
//!   - Action F means to move forward by the given value in the direction the ship is
//!     currently facing.
//!
//!   The ship starts by facing east. Only the L and R actions change the direction the ship
//!   is facing. (That is, if the ship is facing east and the next instruction is N10, the
//!     ship would move north 10 units, but would still move east if the following action were F.)
//!
//! For example:
//! ```skip
//! F10
//! N3
//! F7
//! R90
//! F11
//! ```
//!
//! These instructions would be handled as follows:
//!   - F10 would move the ship 10 units east (because the ship starts by facing east)
//!     to east 10, north 0.
//!   - N3 would move the ship 3 units north to east 10, north 3.
//!   - F7 would move the ship another 7 units east (because the ship is still facing east)
//!     to east 17, north 3.
//!   - R90 would cause the ship to turn right by 90 degrees and face south; it remains at
//!     east 17, north 3.
//!   - F11 would move the ship 11 units south to east 17, south 8.
//!
//!   At the end of these instructions, the ship's Manhattan distance (sum of the absolute values
//!     of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.
//!
//! Figure out where the navigation instructions lead. What is the Manhattan distance between that
//! location and the ship's starting position?
//!
//! --- Part Two ---
//!
//! Before you can give the destination to the captain, you realize that the actual action meanings
//! were printed on the back of the instructions the whole time.
//!
//! Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:
//!   - Action N means to move the waypoint north by the given value.
//!   - Action S means to move the waypoint south by the given value.
//!   - Action E means to move the waypoint east by the given value.
//!   - Action W means to move the waypoint west by the given value.
//!   - Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
//!   - Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
//!   - Action F means to move forward to the waypoint a number of times equal to the given value.
//!
//! The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative
//! to the ship; that is, if the ship moves, the waypoint moves with it.
//!
//! For example, using the same instructions as above:
//!   - F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving
//!     the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
//!   - N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains
//!     at east 100, north 10.
//!   - F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the
//!     ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
//!   - R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units
//!     south of the ship. The ship remains at east 170, north 38.
//!   - F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving
//!     the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.
//!
//! After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.
//!
//! Figure out where the navigation instructions actually lead. What is the Manhattan distance between that
//! location and the ship's starting position?


#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    MoveForward,
    TurnLeft,
    TurnRight,
}

impl Instruction {
    pub fn to_coordinate(self) -> Option<Coordinate> {
        let coordinate: Option<Coordinate> = match self {
            Instruction::MoveNorth => Some(Direction::North.into()),
            Instruction::MoveSouth => Some(Direction::South.into()),
            Instruction::MoveEast => Some(Direction::East.into()),
            Instruction::MoveWest => Some(Direction::West.into()),
            _ => None,
        };
        coordinate
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Angle(isize);

impl Angle {
    pub fn new(mut angle: isize) -> Self {
        angle %= 360;
        if angle < 0 {
            angle += 360;
        }
        Angle(angle)
    }
}

impl std::ops::Add<isize> for Angle {
    type Output = isize;

    fn add(self, rhs: isize) -> Self::Output {
        (self.0 + rhs) % 360
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinate(isize, isize);

impl std::ops::Add<Coordinate> for Coordinate {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0 += rhs.0;
        self.1 += rhs.1;

        self
    }
}

impl std::ops::Mul<isize> for Coordinate {
    type Output = Coordinate;

    fn mul(mut self, rhs: isize) -> Self::Output {
        self.0 *= rhs;
        self.1 *= rhs;

        self
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Into<Coordinate> for Direction {
    fn into(self) -> Coordinate {
        match self {
            Direction::North => Coordinate(0, 1),
            Direction::South => Coordinate(0, -1),
            Direction::East => Coordinate(1, 0),
            Direction::West => Coordinate(-1, 0),
        }
    }
}

impl Into<Angle> for Direction {
    fn into(self) -> Angle {
        match self {
            Direction::North => Angle(0),
            Direction::South => Angle(180),
            Direction::East => Angle(90),
            Direction::West => Angle(270),
        }
    }
}

impl Into<Direction> for Angle {
    fn into(self) -> Direction {
        match self {
            Angle(0) => Direction::North,
            Angle(180) => Direction::South,
            Angle(90) => Direction::East,
            Angle(270) => Direction::West,
            Angle(_) => panic!("wtf if this {:?}", self),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ship {
    coordinates: Coordinate,
    direction: Direction,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            coordinates: Coordinate(0, 0),
            direction: Direction::East,
        }
    }
}

impl Ship {
    pub fn manhattan_distance(&self) -> usize {
        self.coordinates.0.abs() as usize + self.coordinates.1.abs() as usize
    }

    pub fn consume_instruction(&mut self, instruction: Instruction, magnitude: isize) {
        match instruction {
            // turning instructions
            Instruction::TurnLeft => {
                let dir_angle: Angle = self.direction.into();
                self.direction = Angle::new(dir_angle + -magnitude).into();
            },
            Instruction::TurnRight => {
                let dir_angle: Angle = self.direction.into();
                self.direction = Angle::new(dir_angle + magnitude).into();
            },

            // Movement instructions
            Instruction::MoveForward => {
                let dir_coord: Coordinate = self.direction.into();
                self.coordinates = self.coordinates + dir_coord * magnitude;
            },
            // Move E/W/N/S
            _ => {
                let dir_coord: Coordinate = instruction.to_coordinate().unwrap();
                self.coordinates = self.coordinates + dir_coord * magnitude;
            },

        }
    }
}

pub fn parse_instructions(data: &str) -> Vec<(Instruction, isize)> {
    let mut instructions = vec![];

    for line in data.split("\n") {
        let chars: Vec<char> = line.chars().collect();
        let instruction = match chars.get(0).unwrap() {
            'N' => Instruction::MoveNorth,
            'E' => Instruction::MoveEast,
            'W' => Instruction::MoveWest,
            'S' => Instruction::MoveSouth,
            'L' => Instruction::TurnLeft,
            'R' => Instruction::TurnRight,
            'F' => Instruction::MoveForward,
            _  => panic!("invalid instruction: {}", line),
        };
        let magnitude: String = chars[1..].iter().collect();
        let magnitude: isize = magnitude.parse().unwrap();

        instructions.push((instruction, magnitude));
    }

    instructions
}

pub fn part_one(data: &str) -> usize {
    let mut ship = Ship::default();
    for (instruction, magnitude) in parse_instructions(data) {
        ship.consume_instruction(instruction, magnitude);
    }

    ship.manhattan_distance()
}

pub fn part_two(_data: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let answer = part_one(&EXAMPLE_DATA);
        assert_eq!(25, answer);
    }

    #[test]
    fn my_part_one() {
        let answer = part_one(&MY_DATA);
        println!("part one: {}", answer);
        assert_eq!(858, answer);
    }

    #[test]
    fn my_part_two() {
        let answer = part_two(&MY_DATA);
        println!("part two: {}", answer);
    }

    const EXAMPLE_DATA: &str = "F10
N3
F7
R90
F11";

    const MY_DATA: &str = "E2
L180
S4
R90
S1
F49
N2
F18
N2
L180
S5
L90
E3
N2
F11
L180
N5
W3
L180
W2
N5
F80
R90
F89
N1
L180
N2
R180
E4
R90
S1
L90
N5
R180
N2
F17
L90
E2
F58
W5
L90
W3
N3
F78
L90
N4
L90
F15
W1
R90
S1
W4
R90
F41
W4
S4
F37
E5
S1
E3
F19
R90
S1
W4
S2
E2
L180
F51
W5
R90
F76
E2
F40
N4
R180
E5
N3
F72
S4
R90
F99
E3
F76
W5
R90
E2
S5
R180
F76
N4
L180
F10
F83
S1
F46
L90
S5
E1
S1
F14
N4
E1
R180
E1
R180
S3
F52
L90
S4
L90
W3
F18
S2
F81
L180
F76
L180
W1
S2
F73
N2
F77
W1
F28
L180
N2
F76
L180
W5
F61
N4
E2
R180
S2
L90
F14
R180
N5
E4
F11
E1
L90
N3
E3
F58
W3
F72
W1
N2
W5
F44
L90
W4
F37
L90
F4
W5
N3
F57
N4
L90
S2
F43
S4
W3
S5
F84
S2
L90
N1
R180
W3
L180
N2
W3
R90
N2
R90
F66
L90
F73
E4
S2
L270
W2
E2
S4
E1
R90
W1
F49
L270
F70
S3
W1
N2
E1
F65
W3
R90
F27
E5
F80
S4
W5
F68
E5
R90
F94
W2
L90
F37
L180
E1
F38
N2
F15
N4
E3
L90
E1
R90
S1
E5
N4
N2
E4
L270
N4
R90
E1
S4
E4
F71
E2
R180
N5
E3
F17
L90
N2
W1
L90
F62
W1
F85
W1
L180
F33
S4
W1
N5
F81
W2
R90
S2
F49
L180
S5
F4
W4
N3
W1
F17
R90
W4
N4
E4
N3
L90
S2
R90
S3
L90
E4
R90
S5
F88
S1
L180
N5
E5
F55
R90
F81
E5
L180
R90
F55
R90
W5
F13
R90
N5
F58
L180
S5
F27
E4
S3
F42
R90
F39
W3
S3
F31
S4
L90
W1
S3
W3
N4
W2
S3
L90
F61
E1
F23
S2
F31
S3
L180
W1
N1
L90
N3
F81
E2
N1
R90
F64
S4
F88
E1
N5
W1
S3
F10
N5
L90
F58
S1
R90
E3
L90
N4
F94
S1
W1
S4
L90
F51
L180
N4
R90
L90
N4
F66
W2
S3
S3
W4
F68
L90
F42
E1
F43
R90
N3
F20
E1
N3
E4
N3
F4
S4
R180
W1
R270
N3
F86
L90
E5
F84
N3
W3
F16
L90
N2
E3
L90
S5
E5
F53
L270
N2
F91
R90
E5
N4
F57
E5
S5
F61
S4
F89
E3
N3
N5
F3
S5
F59
E5
F66
R180
S1
W1
N2
R180
S4
E2
L90
N1
W2
F13
L90
E5
F6
W3
F78
E1
F7
W1
N4
W5
F58
R90
E4
N3
E5
N3
W1
S3
R90
F16
L90
F93
R270
N5
F2
W1
S3
F54
R270
F18
R180
F95
L90
W1
E4
N2
W1
L90
S2
L90
W2
S4
F92
W2
S3
R180
N5
E3
N5
E5
F22
F88
S3
E2
R90
S5
W1
L90
E4
F77
N1
W3
F14
E3
R90
W1
F21
N1
F58
W4
N2
R90
N2
W4
F68
W5
N3
L90
F22
R90
F90
F84
S5
F30
N1
W4
R90
F17
R90
W4
S5
E2
N1
F92
N2
R180
N5
E2
R90
F38
R90
F15
E5
N4
N4
E4
S4
F92
R90
F22
S3
W4
N3
E1
R180
F96
L90
E1
N1
F9
W2
N4
F17
N2
R90
F76
S2
F5
S5
F34
R90
F7
N4
F83
N5
L90
W1
S3
R90
S3
W2
S3
F51
N5
W4
F8
E3
F10
N5
F39
S3
E2
L90
E5
L90
N5
E2
N3
F42
S3
F38
N5
F19
F97
W2
R180
S4
E4
S2
W3
F39
W4
F70
S1
W1
R90
F41
L90
E1
N1
E3
W5
F13
E4
F2
R180
F27
E4
N2
L270
E1
N3
W4
F81
W3
R90
E1
F57
S5
R90
F13
L180
N5
F98
F32
N3
R90
N3
W3
S3
W3
N4
F73
L180
N1
E4
F7
E4
R90
N4
R90
S2
E5
F32
S2
N5
W3
R90
W5
S2
L90
F4
R270
N5
E3
L90
S5
F24
N4
R90
F27
L90
F16
R90
N2
R90
N3
L90
S3
L90
F85
S3
F47
N1
E1
N3
R270
S2
L90
F50
L90
S2
F23
N4
L180
E3
F91
R90
E1
W4
F81
W5
R90
F46
E1
W1
F91
N5
W5
N3
W1
L90
F60
S2
L90
E1
F82
S3
W5
N5
F90
E3
S1
F61
E4
F98
R180
F8
R270
F73
W4
L90
W5
L90
F86
W5
L180
F61
N5
F88
E2
L270
F90
N5
F21
R270
F40
L90
W1
N2
L90
E2
S5
E2
S1
E5
N3
F51
S1
F58
W3
L180
F13
R90
N1
F79
W2
F61
R90
F22
E2
N5
F1
S4
F99
S1
S3
E2
F97";

}