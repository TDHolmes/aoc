//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
const OUR_INPUT: Result<&str, std::str::Utf8Error> =
    std::str::from_utf8(include_bytes!("../assets/day_09.txt"));

struct HeightMap {
    /// a height map padded on all sides with 0xF
    map: Vec<Vec<u8>>,
    /// A mapping to what basin this coordinate belongs to
    basin_map: Vec<Vec<u8>>,
}

impl HeightMap {
    pub fn from_str(string: &str) -> HeightMap {
        let mut map: Vec<Vec<u8>> = Vec::new();
        let mut basin_map: Vec<Vec<u8>> = Vec::new();
        let first_line = &string[0..string.find("\n").expect("invalid input")];
        let width = first_line.len() + 2; // include the padding on each side

        let mut padding_line = Vec::with_capacity(width);
        padding_line.resize(width, 0xF);

        map.push(padding_line.clone());
        basin_map.push(padding_line.clone());
        for line in string.split_terminator("\n") {
            let mut map_line = Vec::with_capacity(width);
            let mut basin_line = Vec::with_capacity(width);
            map_line.push(0xF);
            basin_line.push(0xF);
            for character in line.chars() {
                map_line.push(character.to_digit(10).expect("invalid char in height map") as u8);
                basin_line.push(0);
            }
            map_line.push(0xF);
            basin_line.push(0xF);
            map.push(map_line);
            basin_map.push(basin_line);
        }
        basin_map.push(padding_line.clone());
        map.push(padding_line);

        Self { map, basin_map }
    }

    pub fn find_minima(&self) -> Vec<u8> {
        let width = self.map[0].len() - 2;
        let height = self.map.len() - 2;
        let mut minima = vec![];

        for y in 1..height + 1 {
            for x in 1..width + 1 {
                let target = &self.map[y][x];
                if target < &self.map[y][x + 1]
                    && target < &self.map[y][x - 1]
                    && target < &self.map[y + 1][x]
                    && target < &self.map[y - 1][x]
                {
                    minima.push(*target);
                }
            }
        }

        minima
    }

    pub fn mark_basins(&mut self) -> Vec<Vec<u8>> {
        let width = self.map[0].len() - 2;
        let height = self.map.len() - 2;
        let mut minima = vec![];

        for y in 1..height + 1 {
            for x in 1..width + 1 {
                let target = &self.map[y][x];
                if target < &self.map[y][x + 1]
                    && target < &self.map[y][x - 1]
                    && target < &self.map[y + 1][x]
                    && target < &self.map[y - 1][x]
                {
                    // this is a minima. We need to now walk from here as long as we walk up "out" of the basin
                    minima.push(*target);
                    let our_name = minima.len() as u8;

                    self.mark_basins_inner(our_name, (x, y));
                }
            }
        }

        self.basin_map.clone()
    }

    fn mark_basins_inner(&mut self, marker: u8, coords: (usize, usize)) {
        if self.map[coords.1][coords.0] >= 9 || self.basin_map[coords.1][coords.0] != 0 {
            return;
        }
        self.basin_map[coords.1][coords.0] = marker;

        self.mark_basins_inner(marker, (coords.0 + 1, coords.1));
        self.mark_basins_inner(marker, (coords.0 - 1, coords.1));
        self.mark_basins_inner(marker, (coords.0, coords.1 + 1));
        self.mark_basins_inner(marker, (coords.0, coords.1 - 1));
    }
}

impl core::fmt::Display for HeightMap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for line in &self.map {
            for c in line {
                write!(f, "{:x}", c)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn part_one(input: &str) -> isize {
    let map = HeightMap::from_str(input);
    println!("{}", map);
    let minima = map.find_minima();
    let risk = minima.iter().map(|v| (v + 1) as isize).sum();

    risk
}

fn part_two(input: &str) -> isize {
    let mut map = HeightMap::from_str(input);
    println!("{}", map);
    let mut basins = map.mark_basins();
    println!("basins: {:?}", basins);
    let mut basin_sums = [0_usize; 256];
    for row in basins {
        for b in row {
            if b != 0xF && b != 0 {
                basin_sums[b as usize] += 1;
            }
        }
    }
    println!("basin sums: {:?}", basin_sums);
    basin_sums.sort();
    let basin_len = basin_sums.len();
    println!(
        "three largest basins: {} {} {}",
        basin_sums[basin_len - 1],
        basin_sums[basin_len - 2],
        basin_sums[basin_len - 3]
    );
    let three_largest_basins =
        basin_sums[basin_len - 1] * basin_sums[basin_len - 2] * basin_sums[basin_len - 3];

    three_largest_basins as isize
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 15);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 1134);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT.unwrap());
    println!("part one: {}", result);
    assert_eq!(600, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT.unwrap());
    println!("part two: {}", result);
    assert_eq!(987840, result);
}
