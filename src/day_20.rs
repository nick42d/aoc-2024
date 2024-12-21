use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    fs::DirEntry,
};

use crate::{
    day_16::{char_to_tile, Tile},
    utils::{Grid, Point},
};

fn parse_input(s: &str) -> Grid<Tile> {
    s.lines().map(|l| l.chars().map(char_to_tile)).collect()
}

#[derive(Copy, Hash, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum CheatTimes {
    Zero,
    One,
    Two,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct DijkstraNode {
    dist: usize,
    cheat: CheatTimes,
    p: Point,
}

fn cheating_dijkstra(
    cheat_enabled: bool,
    g: &Grid<Tile>,
) -> HashMap<Point, HashMap<CheatTimes, usize>> {
    let start = g.find_unchecked(Tile::Start);
    let target = g.find_unchecked(Tile::End);
    let cheat = if cheat_enabled {
        CheatTimes::Zero
    } else {
        CheatTimes::Two
    };
    // Queue keeps track of which visited point has the lowest score so far.
    let mut queue = BinaryHeap::new();
    // Best keeps track of the best score for each point so far.
    let mut best = HashMap::new();
    // Initialisation.
    queue.push(Reverse(DijkstraNode {
        dist: 0,
        p: start,
        cheat,
    }));

    best.insert(start, HashMap::from([(cheat, 0usize)]));

    loop {
        // Loop over all pending nodes in the queue, starting at the current shortest
        // path.
        // We are finished if either:
        // - found goal
        // - no nodes left.
        let Some(Reverse(DijkstraNode {
            dist: next_dist,
            p: next_p,
            cheat: next_cheat,
        })) = queue.pop()
        else {
            break;
        };
        // if next_p == target {
        //     break;
        // }
        // Neighbours are:
        // - In bounds
        // - Non-corrupted
        let next_moves = match next_cheat {
            CheatTimes::Zero => Box::new(
                next_p
                    .adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                    .into_iter()
                    .filter(|n| g.get_cell_unchecked(*n) == &Tile::Wall)
                    .map(|n| {
                        Reverse(DijkstraNode {
                            dist: next_dist + 1,
                            p: n,
                            cheat: CheatTimes::One,
                        })
                    })
                    .chain(
                        next_p
                            .adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                            .into_iter()
                            .filter(|n| g.get_cell_unchecked(*n) != &Tile::Wall)
                            .map(|n| {
                                Reverse(DijkstraNode {
                                    dist: next_dist + 1,
                                    p: n,
                                    cheat: CheatTimes::Zero,
                                })
                            }),
                    ),
            ) as Box<dyn Iterator<Item = Reverse<DijkstraNode>>>,
            CheatTimes::One => Box::new(
                next_p
                    .adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                    .into_iter()
                    .map(|n| {
                        Reverse(DijkstraNode {
                            dist: next_dist + 1,
                            p: n,
                            cheat: CheatTimes::Two,
                        })
                    }),
            ) as Box<dyn Iterator<Item = Reverse<DijkstraNode>>>,
            CheatTimes::Two => Box::new(
                next_p
                    .adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                    .into_iter()
                    .filter(|n| g.get_cell_unchecked(*n) != &Tile::Wall)
                    .map(|n| {
                        Reverse(DijkstraNode {
                            dist: next_dist + 1,
                            p: n,
                            cheat: CheatTimes::Two,
                        })
                    }),
            ) as Box<dyn Iterator<Item = Reverse<DijkstraNode>>>,
        };
        for m in next_moves.into_iter() {
            println!("{:?}", m);
            match best.get(&m.0.p) {
                Some(h) => {
                    if !h.iter().any(|(c, d)| *d < m.0.dist && *c >= m.0.cheat) {
                        // Best distance so far & at lowest or equal lowest cheat level.
                        best.insert(m.0.p, HashMap::from([(m.0.cheat, m.0.dist)]));
                    } else if let Some(mp) = h.get(&m.0.cheat) {
                        if *mp <= m.0.dist {
                            // Been hear before at same distance, same cheat level. Don't need to
                            // revisit.
                            continue;
                        }
                        // Best distance so far at current cheat level.
                        best.get_mut(&m.0.p).unwrap().insert(m.0.cheat, m.0.dist);
                    } else {
                        // Haven't been here at current cheat level before.
                        best.get_mut(&m.0.p).unwrap().insert(m.0.cheat, m.0.dist);
                    }
                    queue.push(m);
                }
                None => {
                    best.insert(m.0.p, HashMap::from([(m.0.cheat, m.0.dist)]));
                    queue.push(m);
                }
            }
        }
    }
    best
}

fn solve_part_1(s: &str, at_least_ps: usize) -> usize {
    let g = parse_input(s);
    cheating_dijkstra(true, &g).len()
}

pub(crate) fn part_1(input: String) {
    println!("Shortest path: {}", solve_part_1(&input, 0));
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        day_16::Tile,
        day_20::{cheating_dijkstra, parse_input, solve_part_1},
    };

    const TEST_DATA: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
    #[test]
    fn test_part_1_no_cheat() {
        let g = parse_input(TEST_DATA);
        let fin = g.find_unchecked(Tile::End);
        let dijkstra = cheating_dijkstra(false, &g);
        assert_eq!(dijkstra.get(&fin).unwrap().values().next().unwrap(), &84);
    }
    #[test]
    fn test_part_1_best_cheat() {
        let g = parse_input(TEST_DATA);
        let fin = g.find_unchecked(Tile::End);
        let dijkstra = cheating_dijkstra(true, &g);
        assert_eq!(dijkstra.get(&fin).unwrap().values().min().unwrap(), &20);
    }
}
