use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

struct Region {
    plant_type: char,
    plots: HashMap<(usize, usize), Plot>,
    sides: usize,
    visited_sides: HashSet<(usize, usize, Direction)>,
}

struct Plot {
    perimiters: usize,
}

#[derive(PartialEq, Default)]
struct Sides {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn rot_90(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn rot_270(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

fn get_plant_type(x: usize, y: usize, grid: &[Vec<char>]) -> Option<char> {
    grid.get(y)?.get(x).copied()
}

fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn check_edges(x: usize, y: usize, plant_type: char, grid: &[Vec<char>]) -> Sides {
    let mut sides = Sides::default();
    if y.checked_sub(1)
        .map(|y| (x, y))
        .and_then(|n| get_plant_type(n.0, n.1, grid))
        != Some(plant_type)
    {
        sides.up = true;
    };
    if Some((x, y + 1)).and_then(|n| get_plant_type(n.0, n.1, grid)) != Some(plant_type) {
        sides.down = true;
    };
    if x.checked_sub(1)
        .map(|x| (x, y))
        .and_then(|n| get_plant_type(n.0, n.1, grid))
        != Some(plant_type)
    {
        sides.left = true
    };
    if Some((x + 1, y)).and_then(|n| get_plant_type(n.0, n.1, grid)) != Some(plant_type) {
        sides.right = true;
    };
    sides
}

fn get_next_edge_position(
    x: usize,
    y: usize,
    direction: Direction,
    plant_type: char,
    grid: &[Vec<char>],
) -> (usize, usize, Direction) {
    let get_offset = |x: usize, y: usize, x_offset, y_offset| {
        y.checked_add_signed(y_offset).and_then(|y| {
            x.checked_add_signed(x_offset)
                .and_then(|x| get_plant_type(x, y, grid))
        })
    };

    match direction {
        Direction::Up => {
            // If can turn 270deg, must turn 270deg
            if get_offset(x, y, 0, -1) == Some(plant_type)
                && get_offset(x, y, -1, -1) == Some(plant_type)
            {
                return (x - 1, y - 1, rot_270(direction));
            }
            // If can't go forward, must turn 90deg
            if get_offset(x, y, 0, -1) != Some(plant_type) {
                return (x, y, rot_90(direction));
            }
            // Otherwise, go forward
            (x, y - 1, direction)
        }
        Direction::Down => {
            // If can turn 270deg, must turn 270deg
            if get_offset(x, y, 0, 1) == Some(plant_type)
                && get_offset(x, y, 1, 1) == Some(plant_type)
            {
                return (x + 1, y + 1, rot_270(direction));
            }
            // If can't go forward, must turn 90deg
            if get_offset(x, y, 0, 1) != Some(plant_type) {
                return (x, y, rot_90(direction));
            }
            // Otherwise, go forward
            (x, y + 1, direction)
        }
        Direction::Left => {
            // If can turn 270deg, must turn 270deg
            if get_offset(x, y, -1, 0) == Some(plant_type)
                && get_offset(x, y, -1, 1) == Some(plant_type)
            {
                return (x - 1, y + 1, rot_270(direction));
            }
            // If can't go forward, must turn 90deg
            if get_offset(x, y, -1, 0) != Some(plant_type) {
                return (x, y, rot_90(direction));
            }
            // Otherwise, go forward
            (x - 1, y, direction)
        }
        Direction::Right => {
            // If can turn 270deg, must turn 270deg
            if get_offset(x, y, 1, 0) == Some(plant_type)
                && get_offset(x, y, 1, -1) == Some(plant_type)
            {
                return (x + 1, y - 1, rot_270(direction));
            }
            // If can't go forward, must turn 90deg
            if get_offset(x, y, 1, 0) != Some(plant_type) {
                return (x, y, rot_90(direction));
            }
            // Otherwise, go forward
            (x + 1, y, direction)
        }
    }
}

/// Returns (number of perimeters of plot, list of unvisited neighbour
/// coordinates)
fn get_perimiters_and_unvisited_neighbours(
    x: usize,
    y: usize,
    plant_type: char,
    grid: &[Vec<char>],
    visited: &HashSet<(usize, usize)>,
) -> (usize, Vec<(usize, usize)>) {
    let neighbours = [
        y.checked_sub(1).map(|y| (x, y)),
        Some((x, y + 1)),
        x.checked_sub(1).map(|x| (x, y)),
        Some((x + 1, y)),
    ];
    let this_perimiters = neighbours
        .iter()
        .filter(|n| n.and_then(|n| get_plant_type(n.0, n.1, grid)) != Some(plant_type))
        .count();
    let unvisited_neghbours = neighbours
        .into_iter()
        .flatten()
        .filter(|n| !visited.contains(n))
        .filter(|n| get_plant_type(n.0, n.1, grid) == Some(plant_type))
        .collect();
    (this_perimiters, unvisited_neghbours)
}

fn visit_location(
    x: usize,
    y: usize,
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) -> Option<char> {
    let plot = get_plant_type(x, y, grid);
    if !visited.insert((x, y)) {
        return None;
    };
    plot
}

fn walk_region(
    x: usize,
    y: usize,
    plant_type: char,
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    region: &mut Region,
) {
    if visit_location(x, y, grid, visited) != Some(plant_type) {
        return;
    }
    let (perimiters, unvisited) =
        get_perimiters_and_unvisited_neighbours(x, y, plant_type, grid, visited);
    if check_edges(x, y, plant_type, grid).left
        && !region.visited_sides.contains(&(x, y, Direction::Up))
    {
        let (sides, visited_sides) = walk_sides(x, y, plant_type, grid);
        region.visited_sides.extend(visited_sides);
        region.sides += sides;
    }
    let insert_result = region.plots.insert((x, y), Plot { perimiters });
    debug_assert!(insert_result.is_none());
    for (x, y) in unvisited {
        walk_region(x, y, plant_type, grid, visited, region);
    }
}

// Returns number of sides, and list of all positions visited.
fn walk_sides(
    mut x: usize,
    mut y: usize,
    plant_type: char,
    grid: &[Vec<char>],
) -> (usize, HashSet<(usize, usize, Direction)>) {
    // Find first left edge.
    debug_assert!(check_edges(x, y, plant_type, grid).left);
    let mut total_sides = 0;
    let mut direction = Direction::Up;
    let mut visited = HashSet::new();
    loop {
        if !visited.insert((x, y, direction)) {
            break;
        }
        let prev_direction = direction;
        (x, y, direction) = get_next_edge_position(x, y, direction, plant_type, grid);
        if direction != prev_direction {
            total_sides += 1
        };
    }
    (total_sides, visited)
}

fn get_regions(s: &str) -> Vec<Region> {
    let grid = parse_input(s);
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, plot) in row.iter().enumerate() {
            let Some(plant_type) = get_plant_type(x, y, &grid) else {
                continue;
            };
            if !visited.contains(&(x, y)) {
                let mut next_region = Region {
                    plant_type,
                    plots: HashMap::new(),
                    sides: 0,
                    visited_sides: HashSet::new(),
                };
                walk_region(x, y, plant_type, &grid, &mut visited, &mut next_region);
                regions.push(next_region);
            }
        }
    }
    regions
}

fn get_total_fencing_price(s: &str) -> usize {
    let regions = get_regions(s);
    let price = regions.iter().fold(0, |acc, e| {
        let (plots, perims) = e.plots.values().fold((0, 0), |(plots, perims), e| {
            (plots + 1, perims + e.perimiters)
        });
        let price = plots * perims;
        acc + price
    });
    price
}

fn get_bulk_discounted_total_fencing_price(s: &str) -> usize {
    let regions = get_regions(s);
    let price = regions.iter().fold(0, |acc, e| {
        let plots = e.plots.len();
        let price = plots * e.sides;
        acc + price
    });
    price
}

pub(crate) fn part_1(input: String) {
    println!("Total fencing price is {}", get_total_fencing_price(&input));
}

pub(crate) fn part_2(input: String) {
    println!(
        "Total fencing price is {}",
        get_bulk_discounted_total_fencing_price(&input)
    );
}

#[cfg(test)]
mod tests {
    use crate::day_12::{
        get_bulk_discounted_total_fencing_price, get_plant_type, get_total_fencing_price,
        parse_input, walk_region, Region,
    };
    use std::collections::{HashMap, HashSet};

    const TEST_DATA_1: &str = "AAAA
BBCD
BBCC
EEEC";
    const TEST_DATA_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const TEST_DATA_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const TEST_DATA_4: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    #[test]
    fn test_part_1_1() {
        assert_eq!(get_total_fencing_price(TEST_DATA_1), 140);
    }
    #[test]
    fn test_part_1_2() {
        assert_eq!(get_total_fencing_price(TEST_DATA_2), 772);
    }
    #[test]
    fn test_part_1_3() {
        assert_eq!(get_total_fencing_price(TEST_DATA_3), 1930);
    }
    #[test]
    fn test_part_2_1() {
        assert_eq!(get_bulk_discounted_total_fencing_price(TEST_DATA_1), 80);
    }
    #[test]
    fn test_part_2_2() {
        assert_eq!(get_bulk_discounted_total_fencing_price(TEST_DATA_4), 368);
    }
}
