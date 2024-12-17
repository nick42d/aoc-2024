use crate::utils::{Direction, Grid, Point};
use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
};

#[derive(PartialEq)]
enum Tile {
    Wall,
    Start,
    End,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Start => write!(f, "S"),
            Tile::End => write!(f, "E"),
            Tile::Empty => write!(f, "."),
        }
    }
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '#' => Tile::Wall,
        'S' => Tile::Start,
        'E' => Tile::End,
        '.' => Tile::Empty,
        _ => unreachable!(),
    }
}

fn parse_input(s: &str) -> Grid<Tile> {
    s.lines().map(|l| l.chars().map(char_to_tile)).collect()
}

fn check_neighbours(
    score: usize,
    pos: Point,
    dir: Direction,
    mut points: HashSet<Point>,
    grid: &Grid<Tile>,
    history: &mut HashMap<Point, (HashMap<Direction, (usize, HashSet<Point>)>)>,
) -> Vec<(usize, Point, Direction, HashSet<Point>)> {
    points.insert(pos);
    match history.entry(pos) {
        Entry::Occupied(mut e) => {
            let prev_visits = e.get_mut();
            match prev_visits.entry(dir) {
                Entry::Occupied(mut e) => {
                    let (prev_score, prev_dirs) = e.get_mut();
                    if *prev_score < score {
                        return vec![];
                    }
                    if *prev_score == score {
                        prev_dirs.extend(points);
                        return vec![];
                    }
                    *prev_score = score;
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert((score, points.clone()));
                }
            }
        }
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert([(dir, (score, points.clone()))].into());
        }
    };
    let free_neighbours = [
        (pos.move_direction(dir), score + 1, dir),
        (pos.move_direction(dir.rot90()), score + 1001, dir.rot90()),
        (pos.move_direction(dir.rot270()), score + 1001, dir.rot270()),
    ]
    .into_iter()
    .filter(|(p, ..)| !matches!(grid.get_cell_unchecked(*p), Tile::Wall));
    let mut out = vec![];
    for (next_pos, next_score, next_dir) in free_neighbours {
        out.extend(check_neighbours(
            next_score,
            next_pos,
            next_dir,
            points.clone(),
            grid,
            history,
        ));
    }
    out
}

fn solve_part_1(s: &str) -> usize {
    let grid = parse_input(s);
    let start_loc = grid.find_unchecked(Tile::Start);
    let end_loc = grid.find_unchecked(Tile::End);
    let mut history = HashMap::new();
    check_neighbours(
        0,
        start_loc,
        Direction::Right,
        HashSet::new(),
        &grid,
        &mut history,
    );
    *history
        .get(&end_loc)
        .unwrap()
        .values()
        .map(|(s, _)| s)
        .min_by(|a, b| a.cmp(b))
        .unwrap()
}

// fn walk_history_dfs(
//     mut cur: Point,
//     end: Point,
//     history: &HashMap<Point, HashMap<Direction, (usize, HashSet<Point>)>>,
// ) -> HashSet<Point> {
//     let mut points = HashSet::new();
//     let mut history_visited = history
//         .iter()
//         .map(|(k, v)| {
//             (
//                 *k,
//                 v.keys()
//                     .map(|v| (*v, false))
//                     .collect::<HashMap<Direction, bool>>(),
//             )
//         })
//         .collect::<HashMap<Point, HashMap<Direction, bool>>>();
//     let mut backtrack_stack = vec![];
//     loop {
//         // Don't exceed cur
//         if cur == end {
//             if let Some(backtrack) = backtrack_stack.pop() {
//                 cur = backtrack
//             } else {
//                 break;
//             }
//         }
//         let mut possible_visits: Vec<_> = history_visited
//             .get_mut(&cur)
//             .unwrap()
//             .iter_mut()
//             .filter(|(_, b)| !**b)
//             .collect();
//         // If there are more than 1 possible visit, add a backtrack to the
// stack.         if possible_visits.len() > 1 {
//             backtrack_stack.push(cur);
//         }
//         // If there are no more possible visits, go to the next backtrack or
// end.         if possible_visits.is_empty() {
//             if let Some(backtrack) = backtrack_stack.pop() {
//                 cur = backtrack;
//                 continue;
//             } else {
//                 break;
//             }
//         }
//         *possible_visits[0].1 = true;
//         cur = cur.move_direction(possible_visits[0].0.rev());
//         points.insert(cur);
//     }
//     points
// }

fn solve_part_2(s: &str) -> usize {
    let grid = parse_input(s);
    let start_loc = grid.find_unchecked(Tile::Start);
    let end_loc = grid.find_unchecked(Tile::End);
    let mut history = HashMap::new();
    check_neighbours(
        0,
        start_loc,
        Direction::Right,
        HashSet::new(),
        &grid,
        &mut history,
    );
    history
        .get(&end_loc)
        .unwrap()
        .values()
        .flat_map(|(_, p)| p.iter())
        .collect::<HashSet<_>>()
        .len()
    // let mut track = walk_history_dfs(end_loc, start_loc, &history);
    // print_data_and_track(&grid, &track);
    // track.len()
}

fn print_data_and_track(m: &Grid<Tile>, h: &HashSet<Point>) {
    for (y, r) in m.repr.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if h.contains(&Point::new(x, y)) {
                if c == &Tile::Wall {
                    panic!();
                }
                print!("O");
                continue;
            }
            print!("{c}");
        }
        println!();
    }
}

pub(crate) fn part_1(input: String) {
    println!("Lowest score is {}", solve_part_1(&input));
}

pub(crate) fn part_2(input: String) {
    println!("Total tiles {}", solve_part_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::day_16::{solve_part_1, solve_part_2};

    const TEST_DATA_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const TEST_DATA_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
    #[test]
    fn test_part_1_1() {
        assert_eq!(solve_part_1(TEST_DATA_1), 7036)
    }
    #[test]
    fn test_part_1_2() {
        assert_eq!(solve_part_1(TEST_DATA_2), 11048)
    }
    #[test]
    fn test_part_2_1() {
        assert_eq!(solve_part_2(TEST_DATA_1), 45)
    }
    #[test]
    fn test_part_2_2() {
        assert_eq!(solve_part_2(TEST_DATA_2), 64)
    }
}
