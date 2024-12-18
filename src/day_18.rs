use std::fmt::Display;

use crate::utils::{Grid, Point};

#[derive(Default)]
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

pub fn part_1_impl(s: &str) -> usize {
    populate_grid(parse_input(s).iter().take(12).copied().collect(), 7, 7).print();
    0
}

pub(crate) fn part_1(input: String) {
    todo!()
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day_18::part_1_impl;

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
        assert_eq!(part_1_impl(TEST_DATA), 22)
    }
}
