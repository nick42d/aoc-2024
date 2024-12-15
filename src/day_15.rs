type Map = Vec<Vec<Tile>>;

enum Tile {
    Empty,
    Robot,
    Box,
    Wall,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            other => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'O' => Tile::Box,
            '.' => Tile::Empty,
            '@' => Tile::Robot,
            '#' => Tile::Wall,
            other => unreachable!("{other}"),
        }
    }
}

fn parse_map(s: &str) -> Map {
    s.lines()
        .map(|line| line.chars().map(Into::into).collect())
        .collect()
}

/// (map, moves)
fn parse_input(s: &str) -> (Map, Vec<Direction>) {
    let (map, moves) = s.split_once("\n\n").unwrap();
    let map = parse_map(map);
    let moves = moves
        .chars()
        .filter(|c| *c != '\n')
        .map(Into::into)
        .collect();
    (map, moves)
}

/// (x, y)
fn robot_coord(m: &Map) -> (usize, usize) {
    for (r_idx, r) in m.iter().enumerate() {
        for (c_idx, t) in r.iter().enumerate() {
            if matches!(t, Tile::Robot) {
                return (c_idx, r_idx);
            }
        }
    }
    unreachable!();
}

fn get_tile(xy: (usize, usize), m: &Map) -> &Tile {
    &m[xy.1][xy.0]
}

fn next_coord(xy: (usize, usize), mve: &Direction) -> (usize, usize) {
    match mve {
        Direction::Up => (xy.0, xy.1 - 1),
        Direction::Down => (xy.0, xy.1 + 1),
        Direction::Left => (xy.0 - 1, xy.1),
        Direction::Right => (xy.0 + 1, xy.1),
    }
}

fn next_empty_tile_coord(
    mut xy: (usize, usize),
    mve: &Direction,
    map: &Map,
) -> Option<(usize, usize)> {
    loop {
        xy = next_coord(xy, mve);
        match get_tile(xy, map) {
            Tile::Box => continue,
            Tile::Wall => return None,
            Tile::Empty => return Some(xy),
            Tile::Robot => unreachable!(),
        }
    }
}

fn apply_move(mut map: Map, mve: Direction) -> Map {
    let robot_pos = robot_coord(&map);
    let maybe_next_pos = next_coord(robot_pos, &mve);
    let next_tile = get_tile(maybe_next_pos, &map);
    match next_tile {
        Tile::Empty => {
            map[maybe_next_pos.1][maybe_next_pos.0] = Tile::Robot;
            map[robot_pos.1][robot_pos.0] = Tile::Empty;
            map
        }
        Tile::Box => {
            let Some(next_empty) = next_empty_tile_coord(maybe_next_pos, &mve, &map) else {
                return map;
            };
            map[maybe_next_pos.1][maybe_next_pos.0] = Tile::Robot;
            map[robot_pos.1][robot_pos.0] = Tile::Empty;
            map[next_empty.1][next_empty.0] = Tile::Box;
            map
        }
        Tile::Wall => map,
        Tile::Robot => unreachable!(),
    }
}

fn total_gps_coords(m: &Map) -> usize {
    m.iter()
        .enumerate()
        .map(|(r_idx, r)| {
            r.iter()
                .enumerate()
                .filter(|(c_idx, c)| matches!(c, Tile::Box))
                .map(|(c_idx, c)| r_idx * 100 + c_idx)
                .reduce(|acc, e| acc + e)
                .unwrap_or_default()
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or_default()
}

fn solve_part_1(s: &str) -> usize {
    let (mut map, moves) = parse_input(s);
    for mve in moves {
        map = apply_move(map, mve);
    }
    total_gps_coords(&map)
}

pub(crate) fn part_1(input: String) {
    println!("Total GPS coords: {}", solve_part_1(&input));
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day_15::{parse_map, solve_part_1, total_gps_coords};

    const TEST_DATA: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
    const SMALL_TEST_DATA: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    #[test]
    fn test() {
        assert_eq!(solve_part_1(TEST_DATA), 10092)
    }
    #[test]
    fn small_test() {
        assert_eq!(solve_part_1(SMALL_TEST_DATA), 2028)
    }
    #[test]
    fn test_total_gps_coords() {
        let map = "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########";
        let map = parse_map(map);
        assert_eq!(total_gps_coords(&map), 10092);
    }
}
