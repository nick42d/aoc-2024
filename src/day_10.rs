use std::collections::VecDeque;

type Grid = Vec<Vec<Option<u32>>>;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
    height: Option<u32>,
}

#[derive(Debug)]
enum NextTrail {
    Init(Coord),
    PartialTrail(Vec<Coord>),
    CompletedTrail(Vec<Coord>),
}

fn get_zeros_loc(g: &Grid) -> Vec<Coord> {
    g.iter()
        .enumerate()
        .flat_map(|(r_idx, row)| {
            row.iter().enumerate().filter_map(move |(c_idx, height)| {
                if let Some(0) = height {
                    return Some(Coord {
                        x: c_idx,
                        y: r_idx,
                        height: *height,
                    });
                }
                None
            })
        })
        .collect()
}

fn get_height(g: &Grid, c: (usize, usize)) -> Option<u32> {
    let (x, y) = c;
    *g.get(y)?.get(x)?
}

fn parse_input(s: &str) -> Grid {
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
        .collect()
}

fn get_next_trails(g: &Grid, c: NextTrail) -> Vec<NextTrail> {
    let mut trail;
    let mut coord;
    match c {
        NextTrail::Init(init_coord) => {
            trail = vec![];
            coord = init_coord;
        }
        NextTrail::PartialTrail(part_trail) => {
            coord = part_trail.last().unwrap().to_owned();
            trail = part_trail;
        }
        NextTrail::CompletedTrail(comp_trail) => unreachable!(),
    };
    let up = (coord.x, coord.y.saturating_sub(1));
    let down = (coord.x, coord.y + 1);
    let left = (coord.x.saturating_sub(1), coord.y);
    let right = (coord.x + 1, coord.y);
    [
        (get_height(g, up), up),
        (get_height(g, down), down),
        (get_height(g, left), left),
        (get_height(g, right), right),
    ]
    .into_iter()
    .filter(|(_, next_coord)| (coord.x, coord.y) != (next_coord.0, next_coord.1))
    .filter(|(next_height, _)| next_height == &coord.height.map(|h| h + 1))
    .map(move |(next_height, next_coord)| {
        let next_coord = Coord {
            x: next_coord.0,
            y: next_coord.1,
            height: next_height,
        };
        let mut trail = trail.clone();
        trail.push(next_coord);
        if let Some(9) = next_height {
            NextTrail::CompletedTrail(trail)
        } else {
            NextTrail::PartialTrail(trail)
        }
    })
    .collect()
}

// Returns a list of all completed trails.
fn recurse_trails(g: &Grid, trails: Vec<NextTrail>) -> Vec<Vec<Coord>> {
    let mut completed = vec![];
    for trail in trails {
        match trail {
            NextTrail::Init(coord) => {
                let next_trails = get_next_trails(g, NextTrail::Init(coord));
                completed.extend(recurse_trails(g, next_trails));
            }
            NextTrail::PartialTrail(trails) => {
                let next_trails = get_next_trails(g, NextTrail::PartialTrail(trails));
                completed.extend(recurse_trails(g, next_trails));
            }
            NextTrail::CompletedTrail(vec) => completed.push(vec),
        }
    }
    completed
}

fn get_trails_score(t: &[Vec<Coord>]) -> usize {
    let mut t = t.to_owned();
    t.sort_by(|t1, t2| t1.last().unwrap().cmp(t2.last().unwrap()));
    t.dedup_by(|t1, t2| t1.last().unwrap() == t2.last().unwrap());
    t.len()
}

fn part_2_solution(s: &str) -> usize {
    let grid = parse_input(s);
    let zeros = get_zeros_loc(&grid);
    let mut output = 0;
    for zero in zeros {
        let next = get_next_trails(&grid, NextTrail::Init(zero));
        let next_completed = recurse_trails(&grid, next);
        output += next_completed.len();
    }
    output
}

fn part_1_solution(s: &str) -> usize {
    let grid = parse_input(s);
    let zeros = get_zeros_loc(&grid);
    let mut output = 0;
    for zero in zeros {
        let next = get_next_trails(&grid, NextTrail::Init(zero));
        let next_completed = recurse_trails(&grid, next);
        output += get_trails_score(&next_completed);
    }
    output
}

pub(crate) fn part_1(input: String) {
    println!("Total trailhead scores: {}", part_1_solution(&input));
}

pub(crate) fn part_2(input: String) {
    println!("Total trailhead scores: {}", part_2_solution(&input));
}

#[cfg(test)]
mod tests {
    use crate::day_10::{parse_input, part_1_solution, part_2_solution};

    const TEST_1: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    const TEST_2: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    const TEST_3: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
    const TEST_4: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn test_part_1_1() {
        let output = part_1_solution(TEST_1);
        assert_eq!(output, 2);
    }
    #[test]
    fn test_part_1_2() {
        let output = part_1_solution(TEST_2);
        assert_eq!(output, 4);
    }
    #[test]
    fn test_part_1_3() {
        let output = part_1_solution(TEST_3);
        assert_eq!(output, 3);
    }
    #[test]
    fn test_part_1_4() {
        let output = part_1_solution(TEST_4);
        assert_eq!(output, 36);
    }
    #[test]
    fn test_part_2_4() {
        let output = part_2_solution(TEST_4);
        assert_eq!(output, 81);
    }
}
