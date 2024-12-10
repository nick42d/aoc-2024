type Grid = Vec<Vec<Option<u32>>>;
// (x, y)
type Coord = (usize, usize);

fn get_coord(g: &Grid, c: Coord) -> Option<u32> {
    let (x, y) = c;
    *g.get(y).and_then(|r| r.get(x))?
}

fn parse_input(s: &str) -> Grid {
    s.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
        .collect()
}

fn get_next_neighbours(g: &Grid, c: Coord) -> Vec<(usize, usize)> {
    get_coord(g, c)
        .map(|height| {
            let up = (c.0, c.1 - 1);
            let down = (c.0, c.1 + 1);
            let left = (c.0 - 1, c.1);
            let right = (c.0 + 1, c.1);
            [
                (get_coord(g, up), up),
                (get_coord(g, down), down),
                (get_coord(g, left), left),
                (get_coord(g, right), right),
            ]
            .into_iter()
            .filter_map(|(next_height, next_coords)| {
                if next_height? == height + 1 {
                    return Some(next_coords);
                }
                None
            })
        })
        .into_iter()
        .flatten()
        .collect()
}

pub(crate) fn part_1(input: String) {
    todo!()
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[cfg(test)]
mod tests {
    const TEST_1: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    const TEST_2: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
    const TEST_3: &str = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
    const TEST_4: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    #[test]
    fn test_part_1_1() {
        let output = todo!();
        assert_eq!(output, 2);
    }
    fn test_part_1_2() {
        let output = todo!();
        assert_eq!(output, 4);
    }
    fn test_part_1_3() {
        let output = todo!();
        assert_eq!(output, 3);
    }
    fn test_part_1_4() {
        let output = todo!();
        assert_eq!(output, 36);
    }
}
