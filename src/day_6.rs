#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Visited(Direction),
    Obstacle,
    HitObstacle(HitDirections),
    Guard(Direction),
}

#[derive(Copy, Clone, PartialEq)]
struct HitDirections {
    from_left: bool,
    from_right: bool,
    from_above: bool,
    from_below: bool,
}

impl HitDirections {
    // Returns true if hit had occurred before.
    fn hit(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::Up => {
                if self.from_below {
                    return true;
                }
                self.from_below = true;
            }
            Direction::Right => {
                if self.from_left {
                    return true;
                }
                self.from_left = true;
            }
            Direction::Down => {
                if self.from_above {
                    return true;
                }
                self.from_above = true;
            }
            Direction::Left => {
                if self.from_right {
                    return true;
                }
                self.from_right = true;
            }
        }
        false
    }
    fn new(direction: Direction) -> Self {
        let mut ret = Self {
            from_left: false,
            from_right: false,
            from_above: false,
            from_below: false,
        };
        match direction {
            Direction::Up => {
                ret.from_below = true;
            }
            Direction::Right => {
                ret.from_left = true;
            }
            Direction::Down => {
                ret.from_above = true;
            }
            Direction::Left => {
                ret.from_right = true;
            }
        }
        ret
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum NextMap {
    InProgress(Vec<Vec<Cell>>),
    Finished(FinishedMap),
}

enum FinishedMap {
    Exited(Vec<Vec<Cell>>),
    Loop(Vec<Vec<Cell>>),
}

impl NextMap {
    fn take_and_is_finished(self) -> (Vec<Vec<Cell>>, bool) {
        match self {
            NextMap::InProgress(vec) => (vec, false),
            NextMap::Finished(f) => match f {
                FinishedMap::Exited(vec) => (vec, true),
                FinishedMap::Loop(vec) => (vec, true),
            },
        }
    }
}

fn turn(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn cell_from_char(c: char) -> Cell {
    match c {
        '.' => Cell::Empty,
        '#' => Cell::Obstacle,
        '>' => Cell::Guard(Direction::Right),
        'v' => Cell::Guard(Direction::Down),
        '<' => Cell::Guard(Direction::Left),
        '^' => Cell::Guard(Direction::Up),
        _ => panic!(),
    }
}

fn char_from_cell(c: &Cell) -> char {
    match c {
        Cell::Empty => '.',
        Cell::Obstacle => '#',
        Cell::HitObstacle { .. } => '@',
        Cell::Guard(Direction::Right) => '>',
        Cell::Guard(Direction::Down) => 'v',
        Cell::Guard(Direction::Left) => '<',
        Cell::Guard(Direction::Up) => '^',
        Cell::Visited(Direction::Right) => '>',
        Cell::Visited(Direction::Down) => 'v',
        Cell::Visited(Direction::Left) => '<',
        Cell::Visited(Direction::Up) => '^',
    }
}

fn text_to_map(text: &str) -> Vec<Vec<Cell>> {
    text.lines()
        .map(|line| line.chars().map(cell_from_char).collect())
        .collect()
}

fn print_map(map: &Vec<Vec<Cell>>) {
    for row in map {
        for cell in row {
            print!("{}", char_from_cell(cell));
        }
        println!();
    }
}

/// Returns the next map iteration.
fn next_map(mut map: Vec<Vec<Cell>>) -> NextMap {
    let mut guard_details = None;
    // Find the guard and their direction.
    'outer: for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if let Cell::Guard(direction) = cell {
                guard_details = Some((*direction, row_idx, col_idx));
                break 'outer;
            }
        }
    }
    let (direction, row_idx, col_idx) = guard_details.unwrap();
    // Set the previous guard location to Visited.
    map[row_idx][col_idx] = Cell::Visited(direction);
    // Get the next grid location - if it's out of range, we are finished.
    let (Some(next_row_idx), Some(next_col_idx)) = (match direction {
        Direction::Up => (row_idx.checked_sub(1), Some(col_idx)),
        Direction::Right => (Some(row_idx), Some(col_idx + 1)),
        Direction::Down => (Some(row_idx + 1), Some(col_idx)),
        Direction::Left => (Some(row_idx), col_idx.checked_sub(1)),
    }) else {
        return NextMap::Finished(FinishedMap::Exited(map));
    };
    if next_row_idx >= map.len() || next_col_idx >= map[0].len() {
        return NextMap::Finished(FinishedMap::Exited(map));
    };
    // Move the guard to the next location, and if we hit an obstacle, add a hit
    // marker.
    match &map[next_row_idx][next_col_idx] {
        Cell::Empty => map[next_row_idx][next_col_idx] = Cell::Guard(direction),
        Cell::Visited(_) => map[next_row_idx][next_col_idx] = Cell::Guard(direction),
        Cell::Obstacle => {
            map[next_row_idx][next_col_idx] = Cell::HitObstacle(HitDirections::new(direction));
            map[row_idx][col_idx] = Cell::Guard(turn(&direction));
        }
        Cell::HitObstacle(mut directions) => {
            if directions.hit(direction) {
                // A loop is recognised when we hit an obstacle in the same location we had hit
                // it previously.
                return NextMap::Finished(FinishedMap::Loop(map));
            }
            map[next_row_idx][next_col_idx] = Cell::HitObstacle(directions);
            map[row_idx][col_idx] = Cell::Guard(turn(&direction));
        }
        Cell::Guard { .. } => unreachable!(),
    };
    NextMap::InProgress(map)
}

fn get_last_map(mut map: Vec<Vec<Cell>>) -> FinishedMap {
    loop {
        map = match next_map(map) {
            NextMap::InProgress(vec) => vec,
            NextMap::Finished(finished_map) => return finished_map,
        }
    }
}

fn count_locations(s: &str) -> usize {
    let mut map = text_to_map(s);
    loop {
        let is_finished;
        (map, is_finished) = next_map(map).take_and_is_finished();
        if is_finished {
            break;
        }
    }
    print_map(&map);
    let mut counter = 0;
    for row in map {
        for cell in row {
            if matches!(cell, Cell::Visited { .. }) {
                counter += 1;
            }
        }
    }
    counter
}

fn count_obstacles_that_cause_loops(s: &str) -> usize {
    let (tx, rx) = std::sync::mpsc::channel();
    let map = text_to_map(s);
    // Run an iteration first to see where the guard visit. We only need to place
    // obstacles on locations that have been visited, so this saves some time.
    let FinishedMap::Exited(exited_map) = get_last_map(map.clone()) else {
        panic!("Initial input shouldn't loop");
    };
    for (row_idx, row) in exited_map.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if !matches!(cell, Cell::Visited(_)) {
                continue;
            }
            let mut test_map = map.clone();
            let tx = tx.clone();
            std::thread::spawn(move || {
                test_map[row_idx][col_idx] = Cell::Obstacle;
                println!("Trying location [{col_idx}, {row_idx}]");
                match get_last_map(test_map) {
                    FinishedMap::Exited(_) => (),
                    FinishedMap::Loop(_) => tx.send(1).unwrap(),
                }
            });
        }
    }
    drop(tx);
    let mut counter = 0;
    while rx.recv().is_ok() {
        counter += 1;
    }
    counter
}

pub(crate) fn part_1(input: String) {
    println!("Guard has visited {} locations.", count_locations(&input));
}

pub(crate) fn part_2(input: String) {
    println!(
        "Obstacles can be placed in {} locations.",
        count_obstacles_that_cause_loops(&input)
    );
}

#[cfg(test)]
mod tests {
    use crate::day_6::{count_locations, count_obstacles_that_cause_loops};

    #[test]
    fn test_count_locations() {
        let test_data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(count_locations(test_data), 41);
    }

    #[test]
    fn test_count_obstacles_that_cause_loops() {
        let test_data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(count_obstacles_that_cause_loops(test_data), 6);
    }
}
