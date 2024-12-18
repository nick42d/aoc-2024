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

#[derive(Debug)]
enum PathType {
    InProgress,
    ReachedGoal,
}

#[derive(Debug)]
enum PathFinder {
    DeadEnd,
    Exploring(Vec<(PathType, usize, Point, Direction, HashSet<Point>)>),
}

fn check_neighbours(
    score: usize,
    pos: Point,
    dir: Direction,
    mut points: HashSet<Point>,
    grid: &Grid<Tile>,
    history: &mut HashMap<Point, (HashMap<Direction, usize>)>,
) -> PathFinder {
    points.insert(pos);
    match history.entry(pos) {
        Entry::Occupied(mut e) => {
            let prev_visits = e.get_mut();
            match prev_visits.entry(dir) {
                Entry::Occupied(mut e) => {
                    let prev_score = e.get_mut();
                    // If we have visited previously with same direction and lower score, stop
                    // exploring this path.
                    if *prev_score < score {
                        return PathFinder::DeadEnd;
                    }
                    *prev_score = score;
                }
                Entry::Vacant(_) => {
                    // If we have visited previously from the reverse direction, same rules as same
                    // direction.
                    if let Some(prev_score) = prev_visits.get_mut(&dir.rev()) {
                        // If we have visited previously with same direction and lower score, stop
                        // exploring this path.
                        if *prev_score < score {
                            return PathFinder::DeadEnd;
                        }
                    }
                    // Otherwise, add a new entry.
                    prev_visits.insert(dir, score);
                }
            }
        }
        Entry::Vacant(vacant_entry) => {
            vacant_entry.insert([(dir, score)].into());
        }
    };
    let free_neighbours = [
        (
            pos.move_direction(dir),
            score + 1,
            dir,
            grid.get_cell_unchecked(pos),
        ),
        (
            pos.move_direction(dir.rot90()),
            score + 1001,
            dir.rot90(),
            grid.get_cell_unchecked(pos),
        ),
        (
            pos.move_direction(dir.rot270()),
            score + 1001,
            dir.rot270(),
            grid.get_cell_unchecked(pos),
        ),
    ]
    .into_iter()
    .filter(|(p, ..)| !matches!(grid.get_cell_unchecked(*p), Tile::Wall));
    let mut out = vec![];
    for (next_pos, next_score, next_dir, next_cell) in free_neighbours {
        if next_cell == &Tile::End {
            out.push((
                PathType::ReachedGoal,
                next_score,
                next_pos,
                next_dir,
                points.clone(),
            ));
            continue;
        }
        let p = check_neighbours(
            next_score,
            next_pos,
            next_dir,
            points.clone(),
            grid,
            history,
        );
        match p {
            PathFinder::DeadEnd => (),
            PathFinder::Exploring(vec) => {
                out.extend(vec);
            }
        }
    }
    PathFinder::Exploring(out)
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
        .min_by(|a, b| a.cmp(b))
        .unwrap()
}

fn solve_part_2(s: &str) -> usize {
    let grid = parse_input(s);
    let start_loc = grid.find_unchecked(Tile::Start);
    let end_loc = grid.find_unchecked(Tile::End);
    let mut history = HashMap::new();
    let neighbours = check_neighbours(
        0,
        start_loc,
        Direction::Right,
        HashSet::new(),
        &grid,
        &mut history,
    );
    match neighbours {
        PathFinder::DeadEnd => todo!(),
        PathFinder::Exploring(vec) => {
            let min_score = vec.iter().map(|x| x.1).min().unwrap();
            vec.iter()
                .filter(|x| x.1 == min_score)
                .flat_map(|x| x.4.iter())
                .copied()
                .collect::<HashSet<_>>()
                .len()
        }
    }
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
