use std::{
    collections::{HashMap, HashSet},
    convert::identity,
};

struct Region {
    plant_type: char,
    plots: HashMap<(usize, usize), Plot>,
}

struct Plot {
    perimiters: usize,
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
    regions: &mut Region,
) {
    if visit_location(x, y, grid, visited) != Some(plant_type) {
        return;
    }
    let (perimiters, unvisited) =
        get_perimiters_and_unvisited_neighbours(x, y, plant_type, grid, visited);
    let insert_result = regions.plots.insert((x, y), Plot { perimiters });
    debug_assert!(insert_result.is_none());
    for (x, y) in unvisited {
        walk_region(x, y, plant_type, grid, visited, regions);
    }
}

fn get_total_fencing_price(s: &str) -> usize {
    let grid = parse_input(s);
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, plot) in row.iter().enumerate() {
            let Some(plant_type) = get_plant_type(x, y, &grid) else {
                continue;
            };
            let mut next_region = Region {
                plant_type,
                plots: HashMap::new(),
            };
            walk_region(x, y, plant_type, &grid, &mut visited, &mut next_region);
            regions.push(next_region);
        }
    }
    let price = regions.iter().fold(0, |acc, e| {
        let (plots, perims) = e.plots.values().fold((0, 0), |(plots, perims), e| {
            (plots + 1, perims + e.perimiters)
        });
        let price = plots * perims;
        acc + price
    });
    price
}

pub(crate) fn part_1(input: String) {
    println!("Total fencing price is {}", get_total_fencing_price(&input));
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day_12::{
        get_plant_type, get_total_fencing_price, parse_input, walk_region, Region,
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
}
