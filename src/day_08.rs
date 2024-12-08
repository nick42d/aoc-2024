use std::collections::HashMap;

#[derive(Debug)]
struct AntennaMap {
    width: usize,
    height: usize,
    antenna_locations: HashMap<char, Vec<(usize, usize)>>,
}

fn parse_input(s: &str) -> AntennaMap {
    let width = s.lines().next().unwrap().len();
    let height = s.lines().count();
    let mut antenna_locations = HashMap::new();
    for (y, row) in s.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antenna_locations
                .entry(c)
                .and_modify(|v: &mut Vec<_>| v.push((x, y)))
                .or_insert(vec![(x, y)]);
        }
    }
    AntennaMap {
        width,
        height,
        antenna_locations,
    }
}

fn get_paired_antinodes_list(
    antennas: &[(usize, usize)],
    width: usize,
    height: usize,
) -> Vec<(isize, isize)> {
    let mut antinodes_list = antennas
        .iter()
        .flat_map(|e| {
            antennas
                .iter()
                .filter(|e2| *e != **e2)
                .flat_map(|e2| get_both_antinodes(*e, *e2))
        })
        .filter(|an| is_in_bounds(*an, width, height))
        .collect::<Vec<_>>();
    antinodes_list.sort();
    antinodes_list.dedup();
    antinodes_list
}

fn get_antinodes_list(
    antennas: &[(usize, usize)],
    width: usize,
    height: usize,
) -> Vec<(isize, isize)> {
    let mut antinodes_list = antennas
        .iter()
        .flat_map(|e| {
            antennas
                .iter()
                .filter(|e2| *e != **e2)
                .flat_map(|e2| get_all_antinodes_in_bounds(*e, *e2, width, height))
        })
        .collect::<Vec<_>>();
    antinodes_list.sort();
    antinodes_list.dedup();
    antinodes_list
}

fn get_both_antinodes(a1: (usize, usize), a2: (usize, usize)) -> [(isize, isize); 2] {
    let a1_x = a1.0 as isize;
    let a1_y = a1.1 as isize;
    let a2_x = a2.0 as isize;
    let a2_y = a2.1 as isize;
    let x_dist = a1_x - a2_x;
    let y_dist = a1_y - a2_y;
    [
        (a2_x - x_dist, a2_y - y_dist),
        (a1_x + x_dist, a1_y + y_dist),
    ]
}

fn get_all_antinodes_in_bounds(
    a1: (usize, usize),
    a2: (usize, usize),
    width: usize,
    height: usize,
) -> Vec<(isize, isize)> {
    let a1_x = a1.0 as isize;
    let a1_y = a1.1 as isize;
    let a2_x = a2.0 as isize;
    let a2_y = a2.1 as isize;
    let x_dist = a1_x - a2_x;
    let y_dist = a1_y - a2_y;
    let mut output = vec![];
    // Axis 1
    for i in 0.. {
        let next_node = (a2_x - x_dist * i, a2_y - y_dist * i);
        if is_in_bounds(next_node, width, height) {
            output.push(next_node)
        } else {
            break;
        }
    }
    // Axis 2
    for i in 0.. {
        let next_node = (a1_x + x_dist * i, a1_y + y_dist * i);
        if is_in_bounds(next_node, width, height) {
            output.push(next_node)
        } else {
            break;
        }
    }
    output
}

// https://en.wikipedia.org/wiki/Greatest_common_divisor#Binary_GCD_algorithm
fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut d = 0;
    while (a % 2) == 0 && (b % 2) == 0 {
        a /= 2;
        b /= 2;
        d += 1;
    }
    while (a % 2) == 0 {
        a /= 2;
    }
    while (b % 2) == 0 {
        b /= 2;
    }
    while a != b {
        if a > b {
            a -= b;
            while (a % 2) == 0 {
                a /= 2;
            }
        }
        if b > a {
            b -= a;
            while (b % 2) == 0 {
                b /= 2;
            }
        }
    }
    2usize.pow(d) * a
}

fn is_in_bounds(antinode: (isize, isize), width: usize, height: usize) -> bool {
    let width = width as isize;
    let height = height as isize;
    antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < width && antinode.1 < height
}

fn unique_paired_antinodes(map: AntennaMap) -> usize {
    let AntennaMap {
        width,
        height,
        antenna_locations,
    } = map;
    let mut antinodes: Vec<_> = antenna_locations
        .values()
        .flat_map(|antennas| get_paired_antinodes_list(antennas, width, height))
        .collect();
    antinodes.sort();
    antinodes.dedup();
    antinodes.len()
}

fn unique_antinodes(map: AntennaMap) -> usize {
    let AntennaMap {
        width,
        height,
        antenna_locations,
    } = map;
    let mut antinodes: Vec<_> = antenna_locations
        .values()
        .flat_map(|antennas| get_antinodes_list(antennas, width, height))
        .collect();
    antinodes.sort();
    antinodes.dedup();
    antinodes.len()
}

pub(crate) fn part_1(input: String) {
    let map = parse_input(&input);
    let unique_antinodes = unique_paired_antinodes(map);
    println!("There are {unique_antinodes} unique antinodes");
}

pub(crate) fn part_2(input: String) {
    let map = parse_input(&input);
    let unique_antinodes = unique_antinodes(map);
    println!("There are {unique_antinodes} unique antinodes");
}

#[cfg(test)]
mod tests {
    use crate::day_08::{
        gcd, get_both_antinodes, get_paired_antinodes_list, parse_input, unique_antinodes,
        unique_paired_antinodes,
    };

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn test_part_1() {
        let map = parse_input(TEST_INPUT);
        let unique_antinodes = unique_paired_antinodes(map);
        assert_eq!(unique_antinodes, 14);
    }
    #[test]
    fn test_part_2() {
        let map = parse_input(TEST_INPUT);
        let unique_antinodes = unique_antinodes(map);
        assert_eq!(unique_antinodes, 34);
    }
    #[test]
    fn test_antinodes_basic() {
        let a1 = (5, 5);
        let a2 = (4, 3);
        let expected = [(3, 1), (6, 7)];
        let output = get_both_antinodes(a1, a2);
        assert_eq!(output, expected);
    }
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
    }
    #[test]
    fn test_get_all_antinodes() {
        let antennas = [(5, 5), (4, 3), (8, 4)];
        let mut expected = [(3, 1), (0, 2), (2, 6), (6, 7)];
        let mut output = get_paired_antinodes_list(&antennas, 10, 10);
        expected.sort();
        output.sort();
        assert_eq!(output, expected);
    }
}
