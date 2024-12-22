//! Utility functions and types that can be shared between days.
use std::{
    borrow::Borrow,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
    ops::Add,
};

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

/// Returns list of all visited locations and the moves taken.
pub fn generic_bfs<T, I, M>(
    init: T,
    goal_check: impl Fn(&T) -> bool,
    // Don't bother gettting neighbours if we are here, already in invalid state.
    state_shortcircuit: impl Fn(&T) -> bool,
    get_neighbours: impl Fn(T) -> I,
) -> HashMap<T, Vec<M>>
where
    T: Eq + Hash + Clone + Debug,
    M: Clone,
    I: IntoIterator<Item = (T, M)>,
{
    let mut queue = VecDeque::new();
    let mut explored = HashMap::new();
    explored.insert(init.clone(), vec![]);
    queue.push_front((init, vec![]));
    while let Some((next_to_visit, mut history)) = queue.pop_back() {
        if goal_check(&next_to_visit) {
            break;
        }
        for (neighbour, m) in get_neighbours(next_to_visit) {
            if !explored.contains_key(&neighbour) && !state_shortcircuit(&neighbour) {
                let mut new_history = history.clone();
                new_history.push(m);
                explored.insert(neighbour.clone(), new_history.clone());
                queue.push_front((neighbour, new_history));
            }
        }
    }
    explored
}

#[derive(Debug)]
struct DijkstraNode<T, W, M> {
    node: T,
    dist: W,
    hist: Vec<M>,
}
impl<T, W, M> DijkstraNode<T, W, M> {
    fn new(val: T, dist: W, hist: impl IntoIterator<Item = M>) -> Self {
        Self {
            node: val,
            dist,
            hist: hist.into_iter().collect(),
        }
    }
    fn equal_to_node(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.node == other.node
    }
}
// Required to implement PartialOrd
impl<T, W, M> PartialEq for DijkstraNode<T, W, M>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}
// Required to implement Ord
impl<T, W, M> Eq for DijkstraNode<T, W, M> where T: Eq {}
impl<T, W, M> PartialOrd for DijkstraNode<T, W, M>
where
    W: PartialOrd,
    T: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}
impl<T, W, M> Ord for DijkstraNode<T, W, M>
where
    W: Ord,
    T: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

/// Returns list of all visited locations, weight to get there, and the moves
/// taken.
pub fn generic_dijkstra<T, I, M, W, R>(
    init: T,
    // Function will short circuit if goal is reached.
    goal_check: impl Fn(&T, &R) -> bool,
    // Don't bother gettting neighbours if we are here, already in invalid state.
    state_shortcircuit: impl Fn(&T, &R) -> bool,
    get_neighbours: impl Fn(T, &R) -> I,
    reference_data: &R,
) -> HashMap<T, (W, Vec<M>)>
where
    T: Eq + Hash + Clone + Debug,
    M: Clone,
    W: Ord + Add<W, Output = W> + Default + Copy + Debug,
    I: IntoIterator<Item = (T, M, W)>,
{
    let mut queue = BinaryHeap::new();
    let mut explored = HashMap::new();
    explored.insert(init.clone(), (W::default(), vec![]));
    queue.push(Reverse(DijkstraNode::new(init, W::default(), vec![])));
    while let Some(Reverse(DijkstraNode { node, dist, hist })) = queue.pop() {
        if goal_check(&node, reference_data) {
            break;
        }
        for (neighbour, m, w) in get_neighbours(node, reference_data) {
            if explored.get(&neighbour).is_none_or(|(e_w, _)| e_w <= &w)
                && !state_shortcircuit(&neighbour, reference_data)
            {
                let mut new_history = hist.clone();
                new_history.push(m);
                explored.insert(neighbour.clone(), (dist + w, new_history.clone()));
                queue.push(Reverse(DijkstraNode::new(neighbour, dist + w, new_history)));
            }
        }
    }
    explored
}
