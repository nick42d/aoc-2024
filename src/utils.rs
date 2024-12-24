//! Utility functions and types that can be shared between days.
use std::{
    borrow::Borrow,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
    ops::Add,
};

pub use algo::*;
mod algo;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

pub struct Grid<T> {
    pub repr: Vec<Vec<T>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn array_plus() -> [Self; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
    pub fn rev(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    /// Clockwise
    pub fn rot90(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
    /// Clockwise
    pub fn rot270(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn move_direction(&self, d: Direction) -> Self {
        match d.borrow() {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
    pub fn adjacent_neighbours(&self) -> [Self; 4] {
        [
            self.move_direction(Direction::Up),
            self.move_direction(Direction::Left),
            self.move_direction(Direction::Right),
            self.move_direction(Direction::Down),
        ]
    }
    pub fn adjacent_inbounds_neighbours(&self, width: usize, height: usize) -> Vec<Self> {
        if self.x == 0 && self.y == 0 {
            return vec![
                self.move_direction(Direction::Right),
                self.move_direction(Direction::Down),
            ];
        }
        if self.x == width - 1 && self.y == height - 1 {
            return vec![
                self.move_direction(Direction::Left),
                self.move_direction(Direction::Up),
            ];
        }
        if self.x == 0 && self.y == height - 1 {
            return vec![
                self.move_direction(Direction::Right),
                self.move_direction(Direction::Up),
            ];
        }
        if self.x == width - 1 && self.y == 0 {
            return vec![
                self.move_direction(Direction::Left),
                self.move_direction(Direction::Down),
            ];
        }
        if self.x > 0 && self.x < width - 1 && self.y > 0 && self.y < height - 1 {
            return vec![
                self.move_direction(Direction::Up),
                self.move_direction(Direction::Left),
                self.move_direction(Direction::Right),
                self.move_direction(Direction::Down),
            ];
        }
        if self.x > 0 && self.x < width - 1 && self.y > 0 {
            return vec![
                self.move_direction(Direction::Up),
                self.move_direction(Direction::Left),
                self.move_direction(Direction::Right),
            ];
        }
        if self.x > 0 && self.x < width - 1 {
            return vec![
                self.move_direction(Direction::Down),
                self.move_direction(Direction::Left),
                self.move_direction(Direction::Right),
            ];
        }
        if self.x > 0 {
            return vec![
                self.move_direction(Direction::Up),
                self.move_direction(Direction::Left),
                self.move_direction(Direction::Down),
            ];
        }
        vec![
            self.move_direction(Direction::Up),
            self.move_direction(Direction::Down),
            self.move_direction(Direction::Right),
        ]
    }
}

impl<T> Grid<T> {
    pub fn get_cell_unchecked(&self, p: Point) -> &T {
        &self.repr[p.y][p.x]
    }
    pub fn get_cell_unchecked_mut(&mut self, p: Point) -> &mut T {
        &mut self.repr[p.y][p.x]
    }
    pub fn get_cell(&self, p: Point) -> Option<&T> {
        self.repr.get(p.y).and_then(|r| r.get(p.x))
    }
    pub fn width_unchecked(&self) -> usize {
        self.repr[0].len()
    }
    pub fn height(&self) -> usize {
        self.repr.len()
    }
}
impl<T: PartialEq> Grid<T> {
    pub fn find_unchecked(&self, val: impl Borrow<T>) -> Point {
        for (r_idx, r) in self.repr.iter().enumerate() {
            for (c_idx, c) in r.iter().enumerate() {
                if c == val.borrow() {
                    return Point::new(c_idx, r_idx);
                }
            }
        }
        panic!("val not found")
    }
}
impl<T: Display> Grid<T> {
    pub fn print(&self) {
        for row in &self.repr {
            for c in row {
                print!("{c}");
            }
            println!();
        }
    }
}
impl<T: Default> Grid<T> {
    pub fn new_with_default(width: usize, height: usize) -> Self {
        let repr = (0..height)
            .map(|_| (0..width).map(|_| T::default()).collect())
            .collect();
        Self { repr }
    }
}

impl<T, A> FromIterator<A> for Grid<T>
where
    A: IntoIterator<Item = T>,
{
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let repr = iter.into_iter().map(|i| i.into_iter().collect()).collect();
        Self { repr }
    }
}
