use crate::utils::{Grid, Point};
use core::net;
use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
    fmt::Display,
    mem::Discriminant,
};

#[derive(Default, PartialEq, Eq)]
enum Byte {
    #[default]
    Ok,
    Corrupted,
}

impl Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Byte::Ok => write!(f, "."),
            Byte::Corrupted => write!(f, "#"),
        }
    }
}

fn parse_input(s: &str) -> Vec<Point> {
    s.lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            Point::new(x, y)
        })
        .collect()
}

fn populate_grid(v: Vec<Point>, width: usize, height: usize) -> Grid<Byte> {
    let mut grid = Grid::new_with_default(width, height);
    for p in v {
        *grid.get_cell_unchecked_mut(p) = Byte::Corrupted;
    }
    grid
}

fn add_byte_to_grid(byte_loc: Point, mut grid: Grid<Byte>) -> Grid<Byte> {
    *grid.get_cell_unchecked_mut(byte_loc) = Byte::Corrupted;
    grid
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct DijkstraNode {
    p: Point,
    dist: usize,
}

fn dijkstra(start: Point, target: Point, g: &Grid<Byte>) -> HashMap<Point, usize> {
    // Queue keeps track of which visited point has the lowest score so far.
    let mut queue = BinaryHeap::new();
    // Best keeps track of the best score for each point so far.
    let mut best = HashMap::new();
    // Initialisation.
    queue.push(Reverse(DijkstraNode { dist: 0, p: start }));
    best.insert(start, 0);

    loop {
        // Loop over all pending nodes in the queue, starting at the current shortest
        // path.
        // We are finished if either:
        // - found goal
        // - no nodes left.
        let Some(Reverse(DijkstraNode {
            dist: next_dist,
            p: next_p,
        })) = queue.pop()
        else {
            break;
        };
        if next_p == target {
            break;
        }
        // Neighbours are:
        // - In bounds
        // - Non-corrupted
        for neighbour in next_p
            .adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
            .into_iter()
            .filter(|n| g.get_cell_unchecked(*n) != &Byte::Corrupted)
            .map(|n| {
                Reverse(DijkstraNode {
                    dist: next_dist + 1,
                    p: n,
                })
            })
        {
            match best.entry(neighbour.0.p) {
                Entry::Occupied(mut occupied_entry) => {
                    if neighbour.0.dist < *occupied_entry.get() {
                        occupied_entry.insert(neighbour.0.dist);
                        queue.retain(|v| v.0.p != neighbour.0.p);
                        queue.push(neighbour);
                    }
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(neighbour.0.dist);
                    queue.push(neighbour);
                }
            }
        }
    }
    best
}

pub fn part_1_impl(s: &str, bytes: usize, width: usize, height: usize) -> usize {
    let grid = populate_grid(
        parse_input(s).iter().take(bytes).copied().collect(),
        width,
        height,
    );
    grid.print();
    let start = Point::new(0, 0);
    let goal = Point::new(width - 1, height - 1);
    let distances = dijkstra(start, goal, &grid);
    *distances.get(&goal).unwrap()
}

pub fn part_2_impl(s: &str, skip: usize, width: usize, height: usize) -> Point {
    let start = Point::new(0, 0);
    let goal = Point::new(width - 1, height - 1);
    let mut seed = parse_input(s);
    let bytes = seed.split_off(skip);
    let mut grid = populate_grid(seed, width, height);
    for byte in bytes {
        grid = add_byte_to_grid(byte, grid);
        if !dijkstra(start, goal, &grid).contains_key(&goal) {
            return byte;
        }
    }
    panic!("No byte found in input that blocks exit");
}

pub(crate) fn part_1(input: String) {
    println!("Shortest path is {}", part_1_impl(&input, 1024, 71, 71));
}

pub(crate) fn part_2(input: String) {
    println!(
        "First byte that blocks exit is {:?}",
        part_2_impl(&input, 1024, 71, 71)
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        day_18::{part_1_impl, part_2_impl},
        utils::Point,
    };

    const TEST_DATA: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
    #[test]
    fn test_part_1() {
        assert_eq!(part_1_impl(TEST_DATA, 12, 7, 7), 22)
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2_impl(TEST_DATA, 12, 7, 7), Point::new(6, 1))
    }
}
