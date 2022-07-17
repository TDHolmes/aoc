//! AOC Day xx
// use aoc_2021;

const EXAMPLE_INPUT: &str = "target area: x=20..30, y=-10..-5";

const OUR_INPUT: &str = "target area: x=119..176, y=-141..-84";

#[derive(Debug)]
struct TargetArea {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl TargetArea {
    fn new(x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> Self {
        Self {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    fn new_from_str(input: &str) -> Self {
        // remove prefix
        let input = input.split("target area: ").nth(1).unwrap();

        let mut x_y_split = input.split(", ");
        let x_range = &x_y_split.next().unwrap()[2..];
        let y_range = &x_y_split.next().unwrap()[2..];

        println!("x_range: {}", x_range);
        println!("y_range: {}", y_range);

        let x_range: Vec<&str> = x_range.split("..").collect();
        let y_range: Vec<&str> = y_range.split("..").collect();

        TargetArea::new(
            x_range[0].parse().unwrap(),
            x_range[1].parse().unwrap(),
            y_range[0].parse().unwrap(),
            y_range[1].parse().unwrap(),
        )
    }

    fn is_within(&self, t: &Trajectory) -> bool {
        t.position.0 >= self.x_min
            && t.position.0 <= self.x_max
            && t.position.1 >= self.y_min
            && t.position.1 <= self.y_max
    }

    fn is_too_far(&self, t: &Trajectory) -> bool {
        t.position.0 > self.x_max || (t.position.1 < self.y_min && t.velocity.1 <= 0)
    }
}

struct Trajectory {
    position: (isize, isize),
    velocity: (isize, isize),
    max_y: isize,
}

impl Trajectory {
    fn new(initial_velocity: (isize, isize)) -> Trajectory {
        Trajectory {
            position: (0, 0),
            velocity: initial_velocity,
            max_y: isize::MIN,
        }
    }

    fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        if self.position.1 > self.max_y {
            self.max_y = self.position.1;
        }

        self.velocity.1 -= 1; // gravity

        // drag
        if self.velocity.0 > 0 {
            self.velocity.0 -= 1;
        } else if self.velocity.0 < 0 {
            self.velocity.0 += 1;
        }
    }
}

fn part_one(input: &str) -> isize {
    let mut max_y = isize::MIN;

    let target = TargetArea::new_from_str(input);
    println!("{:?}", target);

    // this arbitrary brute force tactic doesn't feel good, but works...
    for y_vel in -1000..1000 {
        for x_vel in -1000..1000 {
            // println!("Attempting ({},{})", x_vel, y_vel);
            let mut t = Trajectory::new((x_vel, y_vel));
            loop {
                t.step();
                if target.is_within(&t) {
                    // hit! Check if we have a new high score
                    // println!("hit!");
                    if t.max_y > max_y {
                        max_y = t.max_y;
                    }
                    break;
                }
                if target.is_too_far(&t) {
                    // miss. Move on.
                    // println!("Miss @ {:?}", t.position);
                    break;
                }
            }
        }
    }

    max_y
}

fn part_two(input: &str) -> isize {
    let mut total_trajectories = 0;

    let target = TargetArea::new_from_str(input);
    println!("{:?}", target);

    // this arbitrary brute force tactic doesn't feel good, but works...
    for y_vel in -1000..1000 {
        for x_vel in -1000..1000 {
            // println!("Attempting ({},{})", x_vel, y_vel);
            let mut t = Trajectory::new((x_vel, y_vel));
            loop {
                t.step();
                if target.is_within(&t) {
                    // hit! Check if we have a new high score
                    // println!("hit!");
                    total_trajectories += 1;
                    break;
                }
                if target.is_too_far(&t) {
                    // miss. Move on.
                    // println!("Miss @ {:?}", t.position);
                    break;
                }
            }
        }
    }

    total_trajectories
}

#[test]
fn example_part_one() {
    let result = part_one(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 45);
}

#[test]
fn example_part_two() {
    let result = part_two(EXAMPLE_INPUT);
    println!("example result: {}", result);
    assert_eq!(result, 112);
}

#[test]
fn test_part_one() {
    let result = part_one(OUR_INPUT);
    println!("part one: {}", result);
    assert_eq!(9870, result);
}

#[test]
fn test_part_two() {
    let result = part_two(OUR_INPUT);
    println!("part two: {}", result);
    assert_eq!(5523, result);
}
