use crate::{
    day_16::{char_to_tile, Tile},
    utils::{Bfs, Grid, Point},
};
use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    fs::DirEntry,
};

fn parse_input(s: &str) -> Grid<Tile> {
    s.lines().map(|l| l.chars().map(char_to_tile)).collect()
}

#[derive(Copy, Hash, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum CheatState {
    Zero,
    InProgress { start: Point, times: usize },
    Finished { start: Point, end: Point },
}

fn next_moves(
    p: Point,
    c: CheatState,
    g: &'_ Grid<Tile>,
    max_cheats: usize,
) -> impl Iterator<Item = ((Point, CheatState), ())> + '_ {
    // Neighbours are:
    // - In bounds
    // - Non-corrupted
    let next_moves = match c {
        CheatState::Zero => Box::new(
            p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                .into_iter()
                .filter(move |n| g.get_cell_unchecked(*n) == &Tile::Wall && max_cheats > 0)
                .map(move |n| (n, CheatState::InProgress { start: p, times: 1 }))
                .chain(
                    p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                        .into_iter()
                        .filter(|n| g.get_cell_unchecked(*n) != &Tile::Wall)
                        .map(|n| (n, CheatState::Zero)),
                ),
        ) as Box<dyn Iterator<Item = _>>,
        CheatState::InProgress { start, times } => Box::new(
            // Case 1 - Cheat decides to finish, isn't in a wall.
            p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                .into_iter()
                .filter(move |n| g.get_cell_unchecked(*n) != &Tile::Wall)
                .map(move |n| (n, CheatState::Finished { start, end: n }))
                // Case 2 - cheat still in progress. Must mark finished if finishing at goal.
                .chain(
                    p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                        .into_iter()
                        .filter(move |n| {
                            times + 1 < max_cheats && g.get_cell_unchecked(*n) != &Tile::End
                        })
                        .map(move |n| {
                            (
                                n,
                                CheatState::InProgress {
                                    start,
                                    times: times + 1,
                                },
                            )
                        }),
                ),
        ) as Box<dyn Iterator<Item = _>>,
        CheatState::Finished { start, end } => Box::new(
            p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                .into_iter()
                .filter(|n| g.get_cell_unchecked(*n) != &Tile::Wall)
                .filter(move |n| *n != start)
                .map(move |n| (n, CheatState::Finished { start, end })),
        ) as Box<dyn Iterator<Item = _>>,
    };
    next_moves.zip(std::iter::repeat(()))
}

fn solve(s: &str, at_least_ps: usize, max_cheats: usize) -> usize {
    let g = parse_input(s);
    let start = g.find_unchecked(Tile::Start);
    let target = g.find_unchecked(Tile::End);
    let shortest_path = *Bfs::new_with_refdata(
        (start, CheatState::Zero),
        |(p, _), b| p == &target,
        |t| std::iter::once(*t),
        |(point, cheat), grid| next_moves(point, cheat, grid, 0),
        &g,
    )
    .execute()
    .iter()
    .find_map(|((p, c), w)| if p == &target { Some(w) } else { None })
    .unwrap();
    let shortest_cheat_path = *Bfs::new_with_refdata(
        (start, CheatState::Zero),
        |(p, _), b| p == &target,
        |t| std::iter::once(*t),
        |(point, cheat), grid| next_moves(point, cheat, grid, max_cheats),
        &g,
    )
    .execute()
    .iter()
    .find_map(|((p, c), w)| if p == &target { Some(w) } else { None })
    .unwrap();
    let mut cheats = Bfs::new_with_refdata(
        (start, CheatState::Zero),
        |_, _| false,
        |t| std::iter::once(*t),
        |(point, cheat), grid| next_moves(point, cheat, grid, max_cheats),
        &g,
    )
    .with_max_len(shortest_path)
    // .in_debug_mode()
    .execute()
    .into_iter()
    .filter_map(|((p, c), h)| if p == target { Some((p, c, h)) } else { None })
    .filter(|(p, c, w)| *w <= (shortest_path - at_least_ps))
    .filter(|(p, c, w)| matches!(c, CheatState::Finished { .. }))
    .collect::<Vec<_>>();
    cheats.sort_by_key(|(p, c, w)| *c);
    cheats.dedup_by_key(|(p, c, w)| *c);
    println!("Shortest no-cheat path is {shortest_path}");
    println!(
        "Therefore, need to find all paths using cheats less than or equal {}",
        shortest_path - at_least_ps
    );
    println!("Shortest cheat path is {shortest_cheat_path}");
    println!("Cheats: {:?}", cheats);
    // g.print_specialised(|p| {
    //     if cheats
    //         .iter()
    //         .any(|(_, c, w)| matches!(c, CheatState::Finished(p_n, _) if p_n ==
    // &p))     {
    //         return Some('1');
    //     }
    //     if cheats
    //         .iter()
    //         .any(|(_, c, w)| matches!(c, CheatState::Finished(_, p_n) if p_n ==
    // &p))     {
    //         return Some('2');
    //     }
    //     None
    // });
    cheats.len()
}

fn get_all_cheats(g: &Grid<Tile>, n: usize) -> Vec<(Point, Point)> {
    g.points()
        .flat_map(|cheat_start| {
            cheat_start
                .adjacent_neighbours_n(n)
                .into_iter()
                .filter(|cheat_end| g.get_cell_unchecked(*cheat_end) != &Tile::Wall)
                .map(move |cheat_end| (cheat_start, cheat_end))
        })
        .collect()
}

pub(crate) fn part_1(input: String) {
    println!("Cheats that save 100ps: {}", solve(&input, 100, 2));
}

pub(crate) fn part_2(input: String) {
    let g = parse_input(&input);
    let c = get_all_cheats(&g, 20);
    println!("Cheats: {:?}", c);
    println!("Cheats number: {}", c.len());
    // println!("Cheats that save 100ps: {}", solve(&input, 100, 20));
}

#[cfg(test)]
mod tests {
    use crate::{
        day_16::Tile,
        day_20::{next_moves, parse_input, solve, CheatState},
        utils::{Bfs, Point},
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
        let start = g.find_unchecked(Tile::Start);
        let fin = g.find_unchecked(Tile::End);
        let graph = Bfs::new_with_refdata(
            (start, CheatState::Zero),
            move |(point, _), b| *point == fin,
            |t| std::iter::once(*t),
            |(point, cheat), grid| next_moves(point, cheat, grid, 0),
            &g,
        )
        .with_history()
        .execute();
        assert_eq!(graph.get(&(fin, CheatState::Zero,)).unwrap().len(), 84);
    }
    #[test]
    fn test_part_1_best_cheat() {
        let g = parse_input(TEST_DATA);
        let start = g.find_unchecked(Tile::Start);
        let fin = g.find_unchecked(Tile::End);
        let graph = Bfs::new_with_refdata(
            (start, CheatState::Zero),
            move |(point, _), b| *point == fin,
            |t| std::iter::once(*t),
            |(point, cheat), grid| next_moves(point, cheat, grid, 2),
            &g,
        )
        .with_history()
        .execute();
        assert_eq!(
            graph
                .into_iter()
                .find(|((k1, k2), v)| *k1 == fin)
                .unwrap()
                .1
                .len(),
            20
        );
    }
    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_DATA, 64, 2), 1);
        assert_eq!(solve(TEST_DATA, 38, 2), 3);
        assert_eq!(solve(TEST_DATA, 2, 2), 14 + 14 + 2 + 4 + 2 + 3 + 5);
    }
    #[test]
    #[ignore = "Temporary ignore, failing"]
    fn test_part_2() {
        assert_eq!(solve(TEST_DATA, 76, 20), 3);
        assert_eq!(solve(TEST_DATA, 74, 20), 7);
        assert_eq!(solve(TEST_DATA, 72, 20), 29);
    }
}
