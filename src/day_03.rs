//! --- Day 3: Toboggan Trajectory ---
//!
//! With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan
//! might be easy, it's certainly not safe: there's very minimal steering and the area is covered in
//! trees. You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You
//! make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:
//!
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! These aren't the only trees, though; due to something you read about once involving arboreal genetics
//! and biome stability, the same pattern repeats to the right many times:
//!
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most
//! row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational
//! numbers); start by counting all the trees you would encounter for the slope right 3, down 1:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1. Then, check
//! the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with O where there was an open square and X
//! where there was a tree:
//!
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! In this example, traversing the map using this slope would cause you to encounter 7 trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many
//! trees would you encounter?

fn to_index(map_width: isize, coordinate: (isize, isize)) -> usize {
    (coordinate.0 + coordinate.1 * map_width) as usize
}

pub fn part_one(map: &str, slope: (isize, isize)) -> i32 {
    const TREE: char = '#';

    let mut trees_hit = 0;
    let mut coordinate: (isize, isize) = (0, 0);
    let map_chars: Vec<char> = map.chars().collect();

    // find width for modulo operations when traversing
    let mut width: isize = 0;
    for character in &map_chars {
        if *character == '\n' {
            break;
        }
        width += 1;
    }
    let width = width;  // make width immutable

    // find height by getting the overall len of the str divided by the width, but also accounting for '\n'
    let height = map.len() as isize / (width + 1);

    println!(" width: {}", width);
    println!("height: {}", height);

    // check if we started on a tree (is this valid?)
    if map_chars[0] == TREE {
        trees_hit += 1;
    }

    // now traverse!
    while coordinate.1 != height {
        coordinate.0 = ((coordinate.0 + slope.0) % width).abs();
        coordinate.1 += slope.1;

        print!("({}, {}) -> ", coordinate.0, coordinate.1);
        if map_chars[to_index(width + 1, coordinate)] == TREE {
            println!("HIT");
            trees_hit += 1;
        } else {
            println!("MISS");
        }
    }

    trees_hit
}

pub fn part_two(map: &str) -> usize {
    let mut multiplied_result: usize = 1;
    const SLOPES: [(isize, isize); 5] = [
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2),
    ];

    for slope in &SLOPES {
        multiplied_result *= part_one(map, *slope) as usize;
    }

    multiplied_result
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DATA: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    const MY_DATA: &str = ".....#............#....#####.##
.#.#....#......#....##.........
......#.#.#.....###.#.#........
......#...#.....#####....#..##.
...#............##...###.##....
#.....#...#....#......##....##.
#...#.#....#..#..##.##...#.....
.......#..........#..#..#.#....
.#.....#.#.......#..#...#....#.
#..#.##.#..................###.
...#.#.##...##.###.....#..#...#
..#.#...#............#.......#.
#..#.#..#.#....#...#.#.....#..#
#......##....#..#.#.#........#.
....#..#.#.#.##............#..#
....#..#..#...#.#.##......#...#
##...#...........#.....###.#...
..#...#.#...#.#.....#....##.##.
....##...##.#....#.....#.##....
#........##......#......#.#.#.#
....#.#.#.........##......#....
.#......#...#.....##..#....#..#
....#..#.#.....#..........#..#.
..##...#..##................#.#
.....#....#.#..#......#........
........#..#.#......#.#........
.....#.#....##.###....#...#....
...##.#.......#....###..#......
............##.#..#...#........
#..###..#.....#.####...........
.......##.....#......#......#..
#........##..#.....##.......#.#
#.##...#...#...#......##..#.#.#
......#....##.#.#...#...##....#
#..#....##.#......#.......##...
.#..........#..........#....#.#
#.....##......##....#..........
..#.#.....#.#...#........#.....
...#........#..#..#.##..##.....
......###.....#..#...#.###...##
.##.##.......#.......###...#...
#.#..#.#.#....#.....###..#...##
......#.##..........#.......##.
#..#.#.........#.....##...##...
..#...#....#....###.#......#...
.....#..#.######.....#..#.#....
..#.#.....#.....##.#....##.#.##
...#.#.#....#....##..#..#.#.##.
...........#.#...#..#..####....
.........#####.#.#.#...#.##.#..
.......#...#......#.##.#.##....
....#.....#.....###..........#.
.#.###....##.#..#..........#...
#...#.........##.....####....#.
##....##...#..........#........
...#.#.#.#....#..........#.....
.......#....#......##.......#..
.#.#..#.........#.#.##....#....
..#.............#..##.#.##..###
.#.##..............#..#..##..#.
..##.#..#......#...##..#.##...#
......#..#....#....#....##..#..
...#...##.............#..###...
...##....#.#.##........#.....##
....#.#.......#..###..#....####
#...#...##..#.####..#...##....#
.......#..#.##..#...#.#........
###.#......#..##..#......#.##..
#....#............#.....#......
..##...#..##......#..#....#....
.#..##...#....#.#...#...#..#..#
........#....###...#..##..###.#
.........#....#....#..#.#.#...#
.#....###.##...#.#...........##
..#..#.#..#.#.##..#...##.......
##..#.#.#....#...#..#..........
#..#.......#....#..##...####...
............#.#..........##.##.
#...#..#.#....#..#.#....##.....
......#...#...#.##............#
#.....##..###..#.#..#.#.##..#.#
#..#.#..#......#.......##.#....
##..#.#..#...#......#.##...###.
.#....#..............#....#.#..
..#.#..##....#....#..##........
.#.#...#..#.....#.#..##........
.....#..#.#......#....#.#..#.#.
....#.###...###.#.#.....#......
...........#.#....##....##.....
..#..#.##..........#...#...#..#
.....#.###.#..........#........
....#....##........###...#.....
.#.....##.......#....#..##..###
#.....#...............##......#
#..#.#..#.#.#.....#.#...#......
.##.###...#....#..........##...
.#.......#.....................
.#.#....#...##..#...#...##.....
.#.#...#.......#.......#...#...
....#.#..#.#..#...#....##......
....##.....#.##....#.##..##..##
..#............#...###.##..#...
.#..#.........#.##....#....#..#
.#..##..#..#........#.#.##.#.##
.###.#...#...............#...#.
...#.##.##.#......#...#....##.#
#......##.......##...###....#.#
#..##.....##......#.#.##....#.#
...#.#....#.#.#...........##..#
#.....##......##.#..........##.
###....#.#...#.#..####.........
.##.#.#...##..#.....#..#...#...
#.....#.#......#..........#.#..
..###.##.#...................#.
#.............#..#........#.##.
#.#.#.#..#.....##..##.#....#...
...#...#..#...#..##..##........
...##...##..#...##...........#.
.####..#.#.#.##.#.......#......
...#....#.......#......#.......
.....#.#...#...#..##..#..#.....
......#.....###.#..#..#.#..###.
.#....#....#..#..##.....##...#.
.#.............##.###.#...#.#..
#..#..#......#.###............#
##.#..##....#..........#.#.#...
......#........#...#.......##..
....#.#..#..........#.....#.#..
...#..#...#.#...#........#.....
.....##...#....#.........##.##.
....#...#...#.##.##...#....#...
.#..#.....##......#..#.#..#....
........##...##.##......#.#.#.#
.................#..#.....##.#.
...#.....#...#.........#..#.#.#
....##.#.....#........#...#..#.
#...............#..#.....#...#.
.....#..#....#...#.####.#.#....
####.#..#.##...#....#...##.....
#...##..#...####..#....#.#...#.
..#.......#.##..##...#.#.......
...........##.......#....#..#..
#.##....#...#.....#....##......
....##.#.......#..#...##.......
...#.........##.#..#......#.###
.#..#..#....#.#.##....###..###.
....#.#........##........##....
....########....#.#.#.###.#...#
...#.###.###.##......##.......#
.#...#.###.......#..........#..
..#..##.........#............#.
.......##.#...#...##...#...#..#
#.##....#.#...#.....#..#.#.....
..#........#..#.#.#.#....#.##..
...#...#.#.........#...#.#..##.
#....#......#.#...........#..##
...#.#.#..#...##...#...#...#...
###..........#.#..........#....
..#....#.#.#.#............#.#..
....#...#..###...#.#....#......
#...........####......##.#.....
..#..##.#...#.....#..#.......##
#.....#..###.....#...##..##....
##..###..##...........#.#...#..
.....#......#..............#...
#..#.##.###.......#.......#...#
#........#....##......#.#......
.#.#.#...#.......#........#.##.
#..#..##.....#...#.#.#.#..###..
.#.#....#..#..#.#....##.#.#....
..#.#.........####.#...#.#.###.
....##........##....#........#.
................#..........#...
..#...................###.##..#
.........#..#..#.#...#....#.#.#
......#.....###.....#.#..#...#.
.#.#.....#..##............##...
...##......##.#....#...........
...##..##..###.#...##..........
....###...#..#.#......#......#.
....##..............#..#..#.#..
####.......#...##.##..#.#......
.#......#.....#....###..#....#.
.#.......#...##...#..##.#......
#.......#.......#.#....#.#.#..#
........#..#..#............##.#
#.#...#.#..##..#.......##..#...
...#....#...#..........##..#...
#.#...#.##....###......##....#.
#..#...###........#..#....#..#.
#....#....###....#..#.......#..
....#.#........#.............#.
.#.##........##...#...#...#...#
#.....##.....#.......#.#.#.....
.#.##..........##..#....#......
.#..##.##.#...##....#.#....##..
........#.#.##.#....#.#..#....#
..#...........................#
.#...........#....#....#.#..#..
........##...........#...#...#.
..#.....#..#......#..##.......#
..#....###..###...#.#.#..#....#
#..#.#...#......##......#......
...........#...##..##....##....
#.#......###..#.....#.......#.#
#.....#....#....#.#...#...#....
....#...#.......#....##.#..#...
.####..##......##.#........#..#
..###..#.#.....#...........##..
..##.#.#..#....#..#..#.........
..........#.#.#####...#........
.###......##.#....#........#...
.....#..#..#.#..#.........#....
..#....#...#...#...##..........
....#..##.#.........##.#..##...
##.####..#...#.#...#.....#..###
..#..#...#...#.....##....#..#.#
#..##..#.....#....#.#.....##..#
...#...........##.....#......#.
......#...#.....#.#..###.......
.........#.....###.##..#...#...
.#...#.##...#..........#.#..##.
......#.......##.....#.....##..
........###..........#...#.....
##.......###..###...##...#.....
#.#.............#..#..#.#......
..##........#.###.....#....##..
......#...#......#....##......#
..#.....#...##...#.......#..#..
..#.###..##.##...#....#...##.#.
........##...#..#.#..##.....#.#
.......................#......#
..##.###......#.#.............#
....#...........###............
##...##.....#.......##.......#.
...#..##..##..#.#.###..#......#
........#........#.#..#..#.....
.#......#....##..........#...#.
.##...........##....#..........
.#..#....###.......#....#..##..
.....###..........#....#.#.#...
...#....###.#.#......#......#..
#.#.##.#.....#..#........#...#.
...#.##.........#..#.....#.....
.##...##......##...###...#.....
...#.....#.##..#...#..#........
........#............#.#.#..##.
###...#.....#...#..#........##.
##...#..#.....#.#....#.#.#.....
#..##.......#...#.#...##..#....
#...#.##.....#.#..#.##......#.#
..#......#.#.#.##.##..........#
..#.##......#.#.#..##..........
....#..#....#..#..............#
..........###.....##..#........
...#.....##.....#..#.#..#...##.
.#..##.#..#....#.#......#.##...
...#.....#..#.#...#..#.....#.#.
#...#.#......##...#..#...#....#
..#.......##...#..#.......#...#
#.....#...........##.#.........
.#......##.....####...#.......#
........#..#.....#.......#..#..
....#.#...##..##...#..#....#...
#.#......#...#.#.###.....#.....
..##...#.#........#.##....#.#.#
.#....#......#.#...###.#.......
.......#.#...##....#.#....#....
.....##..##...#..#.#.....##..#.
.##..#.#.#....##.#...#.....#...
.#..#..##....#.##.......#...#..
....#.##...#..##......#.....#..
.#..#....##....#...............
..##...#.....###...............
..............#.#.##........#.#
.#.#....#....#...#.#........#..
.##...#...#.#....#....#.#.....#
#..............#......#.####.#.
......#...........#..#.....##..
#.#..##.##.....#......#..#.#..#
##.##..#.##.#.............#...#
...#..#......#....#............
........###.#.#..#...#.....#.##
..#.......#.##.........#..#....
...##.#........##...#.#.##..#..
...#..#......#...#....#........
...........#..#..#...##...#....
...#.....#....#..####..##.....#
.......#..#..#......#.........#
#......#........###.....##....#
..#..#..#.#.#....##...##......#
#.#..#..###.#..#.....####......
.#................#####....#...
.#.........#...#.......#......#
..#.......#######........#.....
..#........#.....#..#...#..#..#
.#..#.#..#....#.#..##...#..#.#.
..#...........#.#...#.#.##.....
...#.#.#....##.###....#...####.
.....#..#.....#..#.#.........#.
......##...#...###............#
..#.#......###..####..#......#.
###.##.#..#......##.#..##.....#
....###...##............#.#....
..#.....##...#...##....#...#...
#.....#.....#...#...#.#..#.....
####..........##.#.#..#.....##.
...#..........#...#...##..##.#.
..........#.........#.#..#..#..
#....###.....#.#...#.......##.#
#..#.##.....#..........#...#...
...#.#.###.......##..#.....#...
#...#.#..#.............#..#.#..
#........#.................#..#
..#.#....#.#..##.#...#..#....#.
#...#..........#...###....#...#
......#.............#....#....#
#.#.......##.......#.#....##..#
##...#....#.............#..#...
........#...###.##.#..###.#...#
...##...#..#..#...##..##......#
..#.......##....#.#.##....#....
.....#....#..#.#...##.#.#.....#";

    #[test]
    fn example() {
        assert_eq!(7, part_one(EXAMPLE_DATA, (3, 1)));
    }

    #[test]
    fn my_part_one() {
        println!("part one: {}", part_one(MY_DATA, (3, 1)));
    }

    #[test]
    fn my_part_two() {
        println!("part two: {}", part_two(MY_DATA));
    }
}
