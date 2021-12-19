//! --- Day 1: Report Repair ---
//!
//! After saving Christmas five years in a row, you've decided to take a
//! vacation at a nice resort on a tropical island. Surely, Christmas will
//! go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only.
//! The gold coins used there have a little picture of a starfish; the
//! locals just call them stars. None of the currency exchanges seem to
//! have heard of them, but somehow, you'll need to find fifty of these
//! coins by the time you arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available
//! on each day in the Advent calendar; the second puzzle is unlocked when
//! you complete the first. Each puzzle grants one star. Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your
//! expense report (your puzzle input); apparently, something isn't quite
//! adding up.
//!
//! Specifically, they need you to find the two entries that sum to 2020
//! and then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//! ```skip
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//! ```
//!
//! In this list, the two entries that sum to 2020 are 1721 and 299.
//! Multiplying them together produces 1721 * 299 = 514579, so the correct
//! answer is 514579.

pub fn part_one(data: &[i32]) -> i32 {
    for (index, first) in data.iter().enumerate() {
        for second in data[index..].iter() {
            if first + second == 2020 {
                return first * second;
            }
        }
    }
    0
}

pub fn part_two(data: &[i32]) -> i32 {
    for (index, first) in data.iter().enumerate() {
        for (inner_index, second) in data[index..].iter().enumerate() {
            for third in data[inner_index..].iter() {
                if first + second + third == 2020 {
                    return first * second * third;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DATA: [i32; 6] = [1721,979,366,299,675,1456];

    const MY_DATA: [i32; 200] = [
        1695, 1157, 1484, 1717, 622, 1513, 1924, 63, 1461, 1971, 1382, 1587, 1913, 1665,
        1464, 1914, 1637, 1527, 1424, 1361, 1187, 272, 1909, 1448, 1623, 1164, 1931, 1646,
        1096, 1655, 1962, 1961, 1694, 1792, 1989, 1616, 138, 1887, 1357, 1965, 1085, 308,
        2007, 1254, 1179, 1124, 1719, 1467, 1928, 1630, 1676, 1359, 1241, 1511, 1413, 1656,
        1818, 1919, 1422, 1745, 1208, 1609, 1544, 1775, 1154, 1057, 1440, 1242, 1202, 1266,
        1305, 1836, 1760, 1730, 1396, 1315, 1496, 1964, 1300, 1195, 1583, 1607, 1743, 1682,
        1453, 1848, 1320, 1601, 954, 1473, 1847, 1486, 1853, 1668, 1342, 1087, 1139, 1349,
        1568, 1728, 1420, 1233, 1073, 1376, 1658, 1477, 1871, 1958, 1950, 1503, 1758, 1474,
        1203, 1336, 1981, 1309, 1618, 1846, 1974, 1940, 1333, 1119, 1756, 1918, 961, 1307,
        1375, 1346, 1611, 1284, 84, 1754, 1608, 2010, 1341, 1136, 1218, 1882, 1911, 1288, 1930,
        1749, 1952, 1556, 1757, 1761, 1112, 1963, 1186, 1373, 1622, 1973, 1330, 1508, 1222,
        1226, 1389, 1679, 1584, 1237, 1563, 1763, 1998, 1293, 1642, 95, 1661, 1674, 1100, 1262,
        1895, 1548, 1400, 1205, 1435, 1156, 1034, 1577, 1701, 1198, 1173, 1500, 1858, 1809, 1780,
        1412, 1982, 1070, 1523, 1776, 1598, 1113, 1144, 1777, 1313, 1102, 1999, 1405, 1784, 1196
    ];

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