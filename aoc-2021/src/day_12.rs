//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "LP-cb
PK-yk
bf-end
PK-my
end-cb
BN-yk
cd-yk
cb-lj
yk-bf
bf-lj
BN-bf
PK-cb
end-BN
my-start
LP-yk
PK-bf
my-BN
start-PK
yk-EP
lj-BN
lj-start
my-lj
bf-LP";

// const OUR_INPUT: Result<&str, std::str::Utf8Error> =
//     std::str::from_utf8(include_bytes!("../assets/day_xx.txt"));

#[derive(Debug, Clone)]
struct Cave {
    name: &'static str,
    next: Vec<Cave>,
}

impl Cave {
    pub fn new(input: &'static str) -> Self {
        Self {
            name: input,
            next: vec![],
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn is_small(&self) -> bool {
        self.name
            .chars()
            .next()
            .expect("invalid cave")
            .is_ascii_lowercase()
    }

    pub fn add_cave_connection(&mut self, cave: Cave) {
        self.next.push(cave);
    }

    pub fn all_caves(&self) -> CaveIter {
        self.all_caves_inner(vec![self.name.to_owned()])
    }

    fn all_caves_inner(&self, path: Vec<String>) -> CaveIter {
        CaveIter {
            head: self,
            ind: 0,
            next_itr: None,
            complete: false,
            path,
        }
    }
}

struct CaveIter<'a> {
    head: &'a Cave,
    ind: usize,
    next_itr: Option<Box<CaveIter<'a>>>,
    complete: bool,
    path: Vec<String>,
}

/// depth-first iteration of our tree
impl<'a> Iterator for CaveIter<'a> {
    type Item = (&'a Cave, Vec<String>);

    fn next(&mut self) -> Option<Self::Item> {
        // if we've already traversed all of our sub-caves and even yielded ourselves, we're done
        if self.complete {
            return None;
        }

        // if we haven't started iterating on our sub-caves, get that initialized
        if self.next_itr.is_none() {
            if self.head.next.len() != 0 {
                let mut path = self.path.clone();
                path.push(self.head.next[0].name().to_owned());
                self.next_itr = Some(Box::new(self.head.next[0].all_caves_inner(path)));
            } else {
                // we have no sub-caves. Just yield ourselves and complete
                self.complete = true;
                return Some((self.head, self.path.clone()));
            }
        }

        // if we are currently iterating through one of our sub-caves
        if let Some(next) = self.next_itr.as_mut() {
            // and it's not done iterating
            if let Some(next_iter_output) = next.next() {
                // yield its next cave
                return Some(next_iter_output);
            } else {
                // if it's done iterating, check if there's more
                self.ind += 1;
                if let Some(next_cave) = self.head.next.get(self.ind) {
                    // if there is another sub-cave, start its iteration. It is guaranteed to at least yield
                    // itself, or iterate through it's sub-caves as well first.
                    let mut path = self.path.clone();
                    path.push(next_cave.name().to_owned());
                    self.next_itr = Some(Box::new(next_cave.all_caves_inner(path)));
                    return Some(
                        self.next_itr
                            .as_mut()
                            .unwrap()
                            .next()
                            .expect("all cave iters should at least yield themselves"),
                    );
                } else {
                    // all done iterating through sub-caves. Time to yield ourselves
                    self.complete = true;
                    return Some((self.head, self.path.clone()));
                }
            }
        }

        // all cases should have been handled in the above code
        unreachable!();
    }
}

fn part_one(input: &str) -> isize {
    // let mut root_cave = Cave::new("start");
    // let mut orphan_caves: Vec<Cave> = vec![];

    for _line in input.split_terminator("\n") {
        // let mut split = line.split("-");
        // let left = split.next().expect("invalid cave mapping given");
        // let right = split.next().expect("invalid cave mapping given");
        // let right_cave = {
        //     for (ind, cave) in orphan_caves.iter().enumerate() {
        //         if cave.name() == right {
        //             break orphan_caves.remove(index);
        //         }
        //     }
        //     Cave::new(right)
        // };
        // let left_cave = {
        //     for (ind, cave) in orphan_caves.iter().enumerate() {
        //         if cave.name() == left {
        //             break orphan_caves.remove(index);
        //         }
        //     }
        //     Cave::new(left)
        // };
    }

    0
}

fn part_two(_input: &str) -> isize {
    let mut result = 0;
    result += 2;

    result
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 2);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 2);
}

#[test]
fn test_part_one() {
    // let result = part_one(OUR_INPUT.unwrap());
    // println!("part one: {}", result);
    // assert_eq!(42, result);
}

#[test]
fn test_part_two() {
    // let result = part_two(OUR_INPUT.unwrap());
    // println!("part two: {}", result);
    // assert_eq!(42, result);
}

#[cfg(test)]
mod utests {
    use super::*;

    #[test]
    fn check_cave_iteration_order_all_small() {
        // e
        // | \
        // d  c
        //    /\
        //   b  a
        let mut root = Cave::new("e");
        let mut third = Cave::new("c");
        third.add_cave_connection(Cave::new("a"));
        third.add_cave_connection(Cave::new("b"));
        root.add_cave_connection(third);
        root.add_cave_connection(Cave::new("d"));

        let mut cave_iter = root.all_caves();
        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("a", next_cave.name());
        assert_eq!(vec!["e", "c", "a"], path);

        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("b", next_cave.name());
        assert_eq!(vec!["e", "c", "b"], path);

        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("c", next_cave.name());
        assert_eq!(vec!["e", "c"], path);

        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("d", next_cave.name());
        assert_eq!(vec!["e", "d"], path);

        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("e", next_cave.name());
        assert_eq!(vec!["e"], path);

        assert!(cave_iter.next().is_none());
    }

    #[test]
    fn check_cave_iteration_order_some_big() {
        // e
        // | \
        // d  C
        //    /\
        //   b  a
        let mut root = Cave::new("e");
        let mut third = Cave::new("c");
        third.add_cave_connection(Cave::new("a"));
        third.add_cave_connection(Cave::new("b"));
        root.add_cave_connection(third);
        root.add_cave_connection(Cave::new("d"));

        let mut cave_iter = root.all_caves();
        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("a", next_cave.name());
        assert_eq!(vec!["e", "c", "a"], path);

        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("b", next_cave.name());
        assert_eq!(vec!["e", "c", "b"], path);

        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("c", next_cave.name());
        assert_eq!(vec!["e", "c"], path);

        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("d", next_cave.name());
        assert_eq!(vec!["e", "d"], path);

        let (next_cave, path) = cave_iter.next().unwrap();
        assert_eq!("e", next_cave.name());
        assert_eq!(vec!["e"], path);

        assert!(cave_iter.next().is_none());
    }
}
