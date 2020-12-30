//! --- Day 10: Adapter Array ---
//!
//! Patched into the aircraft's data port, you discover weather forecasts of a massive
//! tropical storm. Before you can figure out whether it will impact your vacation plans,
//! however, your device suddenly turns off!
//!
//! Its battery is dead.
//!
//! You'll need to plug it in. There's only one problem: the charging outlet near your seat
//! produces the wrong number of jolts. Always prepared, you make a list of all of the joltage
//! adapters in your bag.
//!
//! Each of your joltage adapters is rated for a specific output joltage (your puzzle input).
//! Any given adapter can take an input 1, 2, or 3 jolts lower than its rating and still
//! produce its rated output joltage.
//!
//! In addition, your device has a built-in joltage adapter rated for 3 jolts higher than the
//! highest-rated adapter in your bag. (If your adapter list were 3, 9, and 6, your device's
//! built-in adapter would be rated for 12 jolts.)
//!
//! Treat the charging outlet near your seat as having an effective joltage rating of 0.
//!
//! Since you have some time to kill, you might as well test all of your adapters. Wouldn't want
//! to get to your resort and realize you can't even charge your device!
//!
//! If you use every adapter in your bag at once, what is the distribution of joltage differences
//! between the charging outlet, the adapters, and your device?
//!
//! For example, suppose that in your bag, you have adapters with the following joltage ratings:
//! ```skip
//! 16
//! 10
//! 15
//! 5
//! 1
//! 11
//! 7
//! 19
//! 6
//! 12
//! 4
//! ```
//!
//! With these adapters, your device's built-in joltage adapter would be rated for 19 + 3 = 22 jolts,
//! 3 higher than the highest-rated adapter.
//!
//! Because adapters can only connect to a source 1-3 jolts lower than its rating, in order to use
//! every adapter, you'd need to choose them like this:
//!   - The charging outlet has an effective rating of 0 jolts, so the only adapters that could connect
//!     to it directly would need to have a joltage rating of 1, 2, or 3 jolts. Of these, only one you
//!     have is an adapter rated 1 jolt (difference of 1).
//!   - From your 1-jolt rated adapter, the only choice is your 4-jolt rated adapter (difference of 3).
//!   - From the 4-jolt rated adapter, the adapters rated 5, 6, or 7 are valid choices. However, in order
//!     to not skip any adapters, you have to pick the adapter rated 5 jolts (difference of 1).
//!   - Similarly, the next choices would need to be the adapter rated 6 and then the adapter rated 7
//!     (with difference of 1 and 1).
//!   - The only adapter that works with the 7-jolt rated adapter is the one rated 10 jolts (difference of 3).
//!   - From 10, the choices are 11 or 12; choose 11 (difference of 1) and then 12 (difference of 1).
//!   - After 12, only valid adapter has a rating of 15 (difference of 3), then 16 (difference of 1), then
//!     19 (difference of 3).
//!   - Finally, your device's built-in adapter is always 3 higher than the highest adapter, so its rating
//!     is 22 jolts (always a difference of 3).
//!
//! In this example, when using every adapter, there are 7 differences of 1 jolt and 5 differences of 3 jolts.
//!
//! Here is a larger example:
//! ```skip
//! 28
//! 33
//! 18
//! 42
//! 31
//! 14
//! 46
//! 20
//! 48
//! 47
//! 24
//! 23
//! 49
//! 45
//! 19
//! 38
//! 39
//! 11
//! 1
//! 32
//! 25
//! 35
//! 8
//! 17
//! 7
//! 9
//! 4
//! 2
//! 34
//! 10
//! 3
//! ```
//!
//! In this larger example, in a chain that uses all of the adapters, there are 22 differences of
//! 1 jolt and 10 differences of 3 jolts.
//!
//! Find a chain that uses all of your adapters to connect the charging outlet to your device's
//! built-in adapter and count the joltage differences between the charging outlet, the adapters,
//! and your device. What is the number of 1-jolt differences multiplied by the number of 3-jolt
//! differences?

use std::collections::HashMap;


pub fn parse_data(data: &str) -> Vec<usize> {
    data.split("\n").map(|line| line.parse().unwrap()).collect()
}


pub fn part_one(data: &str) -> usize {
    let mut input_adapters = parse_data(data);

    // initialize our hashmap, tracking the amount of differences in joltage
    let mut differences: HashMap<isize, usize> = HashMap::new();
    differences.insert(1, 0);
    differences.insert(2, 0);
    differences.insert(3, 0);

    // our device is always 3 jolts higher than the max adapter
    let device_adapter: isize = *input_adapters.iter().max().unwrap() as isize + 3;
    let mut joltage: isize = 0;

    loop {
        let mut difference = device_adapter;
        if input_adapters.len() == 0 {
            panic!("drained input adapters but aren't done?!");
        }

        for adapter in input_adapters.iter() {
            let diff = *adapter as isize - joltage;
            difference = diff.min(difference);
        }
        if difference > 3 {
            panic!("Difference is greater than 3! ({} > 3)", difference);
        }
        *differences.get_mut(&difference).unwrap() += 1;
        input_adapters.retain(|value| *value != (joltage + difference) as usize);
        joltage += difference;
        println!("Difference: {}, Joltage: {} (adapter: {})", difference, joltage, device_adapter);

        if joltage + 3 == device_adapter {
            *differences.get_mut(&3).unwrap() += 1;
            break;
        }
    }

    println!("1 diffs: {}", differences.get(&1).unwrap());
    println!("3 diffs: {}", differences.get(&3).unwrap());

    differences.get(&1).unwrap() * differences.get(&3).unwrap()
}

pub fn part_two(_data: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(220, part_one(&EXAMPLE_DATA));
        assert_eq!(19208, part_two(&EXAMPLE_DATA));
    }

    #[test]
    fn my_part_one() {
        let answer = part_one(&MY_DATA);
        println!("part one: {}", answer);
        assert_eq!(2664, answer);
    }

    #[test]
    fn my_part_two() {
        let answer = part_two(&MY_DATA);
        println!("part two: {}", answer);
    }

    const EXAMPLE_DATA: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    const MY_DATA: &str = "147
174
118
103
67
33
96
28
43
22
16
138
75
148
35
6
10
169
129
115
21
52
58
79
46
7
139
104
91
51
172
57
49
126
95
149
125
123
112
30
78
44
37
167
157
29
173
98
36
63
111
160
18
8
9
159
179
72
110
2
53
150
17
81
97
108
102
56
135
166
168
163
1
25
3
158
101
132
144
45
140
34
156
178
105
68
153
80
82
59
50
122
69
85
109
40
124
119
94
88
13
180
177
133
66
134
60
141";

}