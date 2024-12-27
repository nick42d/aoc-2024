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
    One(Point),
    Two(Point, Point),
}

fn next_moves(
    p: Point,
    c: CheatState,
    g: &'_ Grid<Tile>,
) -> impl Iterator<Item = ((Point, CheatState), ())> + '_ {
    // Neighbours are:
    // - In bounds
    // - Non-corrupted
    let next_moves = match c {
        CheatState::Zero => Box::new(
            p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                .into_iter()
                .filter(|n| g.get_cell_unchecked(*n) == &Tile::Wall)
                .map(|n| (n, CheatState::One(n)))
                .chain(
                    p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                        .into_iter()
                        .filter(|n| g.get_cell_unchecked(*n) != &Tile::Wall)
                        .map(|n| (n, CheatState::Zero)),
                ),
        ) as Box<dyn Iterator<Item = _>>,
        CheatState::One(c1) => Box::new(
            p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                .into_iter()
                .map(move |n| (n, CheatState::Two(c1, n))),
        ) as Box<dyn Iterator<Item = _>>,
        CheatState::Two(c1, c2) => Box::new(
            p.adjacent_inbounds_neighbours(g.width_unchecked(), g.height())
                .into_iter()
                .filter(|n| g.get_cell_unchecked(*n) != &Tile::Wall)
                .filter(move |n| *n != c1)
                .map(move |n| (n, CheatState::Two(c1, c2))),
        ) as Box<dyn Iterator<Item = _>>,
    };
    next_moves.zip(std::iter::repeat(()))
}

fn solve_part_1(s: &str, at_least_ps: usize) -> usize {
    let g = parse_input(s);
    let start = g.find_unchecked(Tile::Start);
    let target = g.find_unchecked(Tile::End);
    let shortest_path = *Bfs::new_with_refdata(
        (start, CheatState::Two(Point::new(0, 0), Point::new(0, 0))),
        |(p, _), b| p == &target,
        |t| std::iter::once(*t),
        |(point, cheat), grid| next_moves(point, cheat, grid),
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
        |(point, cheat), grid| next_moves(point, cheat, grid),
        &g,
    )
    .execute()
    .iter()
    .find_map(|((p, c), w)| if p == &target { Some(w) } else { None })
    .unwrap();
    let mut cheats = Bfs::new_with_refdata(
        (start, CheatState::Zero),
        |_, _| false,
        |(p, c)| match c {
            CheatState::Zero => vec![(*p, CheatState::Zero)],
            CheatState::One(point) => vec![(*p, CheatState::One(*point)), (*p, CheatState::Zero)],
            CheatState::Two(point, point1) => vec![
                (*p, CheatState::Two(*point, *point1)),
                (*p, CheatState::Zero),
            ],
        },
        |(point, cheat), grid| next_moves(point, cheat, grid),
        &g,
    )
    .with_max_len(shortest_path)
    .in_debug_mode()
    .execute()
    .into_iter()
    .filter_map(|((p, c), h)| if p == target { Some((p, c, h)) } else { None })
    .filter(|(p, c, w)| *w < (shortest_path - 100))
    .filter(|(p, c, w)| matches!(c, CheatState::Two(_, _)))
    .collect::<Vec<_>>();
    cheats.sort_by_key(|(p, c, w)| *c);
    cheats.dedup_by_key(|(p, c, w)| *c);
    println!("Shortest no-cheat path is {shortest_path}");
    println!(
        "Therefore, need to find all paths using cheats less than or equal {}",
        shortest_path - 100
    );
    println!("Shortest cheat path is {shortest_cheat_path}");
    println!("Cheats: {:?}", cheats);
    g.print_specialised(|p| {
        if cheats
            .iter()
            .any(|(_, c, w)| matches!(c, CheatState::Two(p_n, _) if p_n == &p))
        {
            return Some('1');
        }
        if cheats
            .iter()
            .any(|(_, c, w)| matches!(c, CheatState::Two(_, p_n) if p_n == &p))
        {
            return Some('2');
        }
        None
    });
    cheats.len()
}

pub(crate) fn part_1(input: String) {
    println!("Cheats that save 100ps: {}", solve_part_1(&input, 0));
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{
        day_16::Tile,
        day_20::{next_moves, parse_input, solve_part_1, CheatState},
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
            (start, CheatState::Two(Point::new(0, 0), Point::new(0, 0))),
            move |(point, _), b| *point == fin,
            |t| std::iter::once(*t),
            |(point, cheat), grid| next_moves(point, cheat, grid),
            &g,
        )
        .with_history()
        .execute();
        assert_eq!(
            graph
                .get(&(fin, CheatState::Two(Point::new(0, 0), Point::new(0, 0))))
                .unwrap()
                .len(),
            84
        );
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
            |(point, cheat), grid| next_moves(point, cheat, grid),
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
}
