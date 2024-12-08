struct Text {
    rows: Vec<Vec<char>>,
}

struct Box<const R: usize, const C: usize> {
    rows: [[char; C]; R],
    source_row: usize,
    source_col: usize,
}

struct BoxesIterator<const R: usize, const C: usize> {
    text: Text,
    next_col: usize,
    next_row: usize,
    width: usize,
    height: usize,
}

impl<const R: usize, const C: usize> Iterator for BoxesIterator<R, C> {
    type Item = Box<R, C>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_row + R > self.height {
            return None;
        }
        let box_rows = self.text.rows[self.next_row..self.next_row + R]
            .iter()
            .map(|r| r[self.next_col..self.next_col + C].try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let out = Box {
            rows: box_rows,
            source_row: self.next_row,
            source_col: self.next_col,
        };
        self.next_col += 1;
        if self.next_col + C > self.width {
            self.next_col = 0;
            self.next_row += 1;
        }
        Some(out)
    }
}

impl<const R: usize, const C: usize> BoxesIterator<R, C> {
    fn new(grid: Text) -> Self {
        Self {
            width: grid.rows.len(),
            height: grid.rows[0].len(),
            text: grid,
            next_col: 0,
            next_row: 0,
        }
    }
}

fn count_xmas(bx: Box<4, 4>) -> usize {
    let Box {
        rows,
        source_row,
        source_col,
    } = bx;
    let is_xmas = |arr| arr == ['X', 'M', 'A', 'S'] || arr == ['S', 'A', 'M', 'X'];
    let mut count = 0;
    if source_row == 0 {
        for r in rows[0..3].iter() {
            if is_xmas([r[0], r[1], r[2], r[3]]) {
                count += 1;
            }
        }
    }
    if source_col == 0 {
        for i in 0..3 {
            if is_xmas([rows[0][i], rows[1][i], rows[2][i], rows[3][i]]) {
                count += 1;
            }
        }
    }
    if is_xmas([rows[3][0], rows[3][1], rows[3][2], rows[3][3]]) {
        count += 1;
    }
    if is_xmas([rows[0][3], rows[1][3], rows[2][3], rows[3][3]]) {
        count += 1;
    }
    if is_xmas([rows[0][0], rows[1][1], rows[2][2], rows[3][3]]) {
        count += 1;
    }
    if is_xmas([rows[0][3], rows[1][2], rows[2][1], rows[3][0]]) {
        count += 1;
    }
    count
}

fn is_x_mas(bx: Box<3, 3>) -> bool {
    let Box { rows, .. } = bx;
    let is_mas = |arr| arr == ['M', 'A', 'S'] || arr == ['S', 'A', 'M'];
    is_mas([rows[0][0], rows[1][1], rows[2][2]]) && is_mas([rows[0][2], rows[1][1], rows[2][0]])
}

pub fn part_1(s: String) {
    let grid = Text {
        rows: s.lines().map(|s| s.chars().collect()).collect(),
    };
    let iter = BoxesIterator::<4, 4>::new(grid);
    let count = iter.fold(0, |acc, e| acc + count_xmas(e));
    println!("{count}");
}

pub fn part_2(s: String) {
    let grid = Text {
        rows: s.lines().map(|s| s.chars().collect()).collect(),
    };
    let iter = BoxesIterator::<3, 3>::new(grid);
    let count = iter.fold(0, |acc, e| acc + is_x_mas(e) as usize);
    println!("{count}");
}
