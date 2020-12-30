

pub fn part_one(_data: &str) -> i32 {
    0
}

pub fn part_two(_data: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DATA: &str = "";

    const MY_DATA: &str = "";

    #[test]
    fn example() {
        assert_eq!(514579, part_one(&EXAMPLE_DATA));
    }

    #[test]
    fn my_part_one() {
        println!("part one: {}", part_one(&MY_DATA));
    }

    #[test]
    fn my_part_two() {
        println!("part two: {}", part_two(&MY_DATA));
    }
}