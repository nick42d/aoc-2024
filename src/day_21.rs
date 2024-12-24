use crate::utils::{
    generic_bfs, generic_bfs_nohistory, generic_dfs_nohistory, generic_dijkstra, Direction,
};
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State<const N: usize> {
    sequence: Vec<NumericKeypadState>,
    // Robot 0 has controls to numeric keypad
    numeric_robot: NumericKeypadState,
    // Starting at robot idx 0, each robot has access to subsequent robots directional keypad.
    //
    // Last robot has controls to numeric_robot.
    // You have controls to robot idx 0 directional keypad.
    directional_robots: [DirectionalKeypadState; N],
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        State {
            sequence: vec![],
            numeric_robot: Default::default(),
            directional_robots: [DirectionalKeypadState::default(); N],
        }
    }
}

impl<const N: usize> State<N> {
    fn get_neighbours(self, _: &()) -> impl Iterator<Item = (Self, DirectionalKeypadState)> {
        let seq_len = self.sequence.len();
        [
            DirectionalKeypadState::A,
            DirectionalKeypadState::Up,
            DirectionalKeypadState::Down,
            DirectionalKeypadState::Left,
            DirectionalKeypadState::Right,
        ]
        .into_iter()
        .filter_map(move |d| Some((self.clone().press_directional_n(d, 0)?, d)))
    }
    /// If a failure is encountered, returns the index.
    fn test_seq(
        mut self,
        seq: impl IntoIterator<Item = DirectionalKeypadState>,
    ) -> Result<Self, usize> {
        for (i, d) in seq.into_iter().enumerate() {
            self = self.press_directional_n(d, 0).ok_or(i)?;
        }
        Ok(self)
    }
    fn press_directional_n(mut self, d: DirectionalKeypadState, n: usize) -> Option<Self> {
        if n == N {
            match d {
                DirectionalKeypadState::A => {
                    self.sequence.push(self.numeric_robot);
                    return Some(self);
                }
                dir => {
                    self.numeric_robot = self.numeric_robot.move_arm(dir.get_dir_unchecked())?;
                    return Some(self);
                }
            }
        }
        match d {
            DirectionalKeypadState::A => {
                let next_dir = self.directional_robots[n];
                self.press_directional_n(next_dir, n + 1)
            }
            dir => {
                self.directional_robots[n] =
                    self.directional_robots[n].move_arm(dir.get_dir_unchecked())?;
                Some(self)
            }
        }
    }
}

#[derive(Hash, Debug, Clone, Copy, Default, PartialEq, PartialOrd, Ord, Eq)]
enum NumericKeypadState {
    #[default]
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl NumericKeypadState {
    fn move_arm(&self, d: Direction) -> Option<Self> {
        match (self, d) {
            (NumericKeypadState::A, Direction::Up) => Some(NumericKeypadState::Three),
            (NumericKeypadState::A, Direction::Left) => Some(NumericKeypadState::Zero),
            (NumericKeypadState::A, _) => None,
            (NumericKeypadState::Zero, Direction::Up) => Some(NumericKeypadState::Two),
            (NumericKeypadState::Zero, Direction::Right) => Some(NumericKeypadState::A),
            (NumericKeypadState::Zero, _) => None,
            (NumericKeypadState::One, Direction::Up) => Some(NumericKeypadState::Four),
            (NumericKeypadState::One, Direction::Right) => Some(NumericKeypadState::Two),
            (NumericKeypadState::One, _) => None,
            (NumericKeypadState::Two, Direction::Up) => Some(NumericKeypadState::Five),
            (NumericKeypadState::Two, Direction::Down) => Some(NumericKeypadState::Zero),
            (NumericKeypadState::Two, Direction::Left) => Some(NumericKeypadState::One),
            (NumericKeypadState::Two, Direction::Right) => Some(NumericKeypadState::Three),
            (NumericKeypadState::Three, Direction::Up) => Some(NumericKeypadState::Six),
            (NumericKeypadState::Three, Direction::Left) => Some(NumericKeypadState::Two),
            (NumericKeypadState::Three, Direction::Down) => Some(NumericKeypadState::A),
            (NumericKeypadState::Three, Direction::Right) => None,
            (NumericKeypadState::Four, Direction::Up) => Some(NumericKeypadState::Seven),
            (NumericKeypadState::Four, Direction::Down) => Some(NumericKeypadState::One),
            (NumericKeypadState::Four, Direction::Right) => Some(NumericKeypadState::Five),
            (NumericKeypadState::Four, Direction::Left) => None,
            (NumericKeypadState::Five, Direction::Up) => Some(NumericKeypadState::Eight),
            (NumericKeypadState::Five, Direction::Down) => Some(NumericKeypadState::Two),
            (NumericKeypadState::Five, Direction::Left) => Some(NumericKeypadState::Four),
            (NumericKeypadState::Five, Direction::Right) => Some(NumericKeypadState::Six),
            (NumericKeypadState::Six, Direction::Up) => Some(NumericKeypadState::Nine),
            (NumericKeypadState::Six, Direction::Down) => Some(NumericKeypadState::Three),
            (NumericKeypadState::Six, Direction::Left) => Some(NumericKeypadState::Five),
            (NumericKeypadState::Six, Direction::Right) => None,
            (NumericKeypadState::Seven, Direction::Down) => Some(NumericKeypadState::Four),
            (NumericKeypadState::Seven, Direction::Right) => Some(NumericKeypadState::Eight),
            (NumericKeypadState::Seven, _) => None,
            (NumericKeypadState::Eight, Direction::Down) => Some(NumericKeypadState::Five),
            (NumericKeypadState::Eight, Direction::Left) => Some(NumericKeypadState::Seven),
            (NumericKeypadState::Eight, Direction::Right) => Some(NumericKeypadState::Nine),
            (NumericKeypadState::Eight, Direction::Up) => None,
            (NumericKeypadState::Nine, Direction::Down) => Some(NumericKeypadState::Six),
            (NumericKeypadState::Nine, Direction::Left) => Some(NumericKeypadState::Eight),
            (NumericKeypadState::Nine, _) => None,
        }
    }
}

#[derive(Hash, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
enum DirectionalKeypadState {
    #[default]
    A,
    Up,
    Down,
    Left,
    Right,
}
impl DirectionalKeypadState {
    fn move_arm(&self, d: Direction) -> Option<Self> {
        match (self, d) {
            (DirectionalKeypadState::A, Direction::Down) => Some(DirectionalKeypadState::Right),
            (DirectionalKeypadState::A, Direction::Left) => Some(DirectionalKeypadState::Up),
            (DirectionalKeypadState::A, _) => None,
            (DirectionalKeypadState::Up, Direction::Down) => Some(DirectionalKeypadState::Down),
            (DirectionalKeypadState::Up, Direction::Right) => Some(DirectionalKeypadState::A),
            (DirectionalKeypadState::Up, _) => None,
            (DirectionalKeypadState::Down, Direction::Up) => Some(DirectionalKeypadState::Up),
            (DirectionalKeypadState::Down, Direction::Left) => Some(DirectionalKeypadState::Left),
            (DirectionalKeypadState::Down, Direction::Right) => Some(DirectionalKeypadState::Right),
            (DirectionalKeypadState::Down, Direction::Down) => None,
            (DirectionalKeypadState::Left, Direction::Right) => Some(DirectionalKeypadState::Down),
            (DirectionalKeypadState::Left, _) => None,
            (DirectionalKeypadState::Right, Direction::Up) => Some(DirectionalKeypadState::A),
            (DirectionalKeypadState::Right, Direction::Left) => Some(DirectionalKeypadState::Down),
            (DirectionalKeypadState::Right, _) => None,
        }
    }
    fn get_dir_unchecked(&self) -> Direction {
        match self {
            DirectionalKeypadState::A => panic!("State is A, not a direction"),
            DirectionalKeypadState::Up => Direction::Up,
            DirectionalKeypadState::Down => Direction::Down,
            DirectionalKeypadState::Left => Direction::Left,
            DirectionalKeypadState::Right => Direction::Right,
        }
    }
}

fn parse_directional(s: &str) -> impl Iterator<Item = DirectionalKeypadState> + '_ {
    s.chars().map(|c| match c {
        'A' => DirectionalKeypadState::A,
        '^' => DirectionalKeypadState::Up,
        'v' => DirectionalKeypadState::Down,
        '<' => DirectionalKeypadState::Left,
        '>' => DirectionalKeypadState::Right,
        _ => unreachable!(),
    })
}

fn get_numeric_code(s: &str) -> usize {
    s.trim_end_matches("A").parse().unwrap()
}

fn parse_numeric(s: &str) -> Vec<NumericKeypadState> {
    s.chars()
        .map(|c| match c {
            'A' => NumericKeypadState::A,
            '0' => NumericKeypadState::Zero,
            '1' => NumericKeypadState::One,
            '2' => NumericKeypadState::Two,
            '3' => NumericKeypadState::Three,
            '4' => NumericKeypadState::Four,
            '5' => NumericKeypadState::Five,
            '6' => NumericKeypadState::Six,
            '7' => NumericKeypadState::Seven,
            '8' => NumericKeypadState::Eight,
            '9' => NumericKeypadState::Nine,
            _ => unreachable!(),
        })
        .collect()
}

fn print_dirs(v: &[DirectionalKeypadState]) {
    for d in v {
        match d {
            DirectionalKeypadState::A => print!("A"),
            DirectionalKeypadState::Up => print!("^"),
            DirectionalKeypadState::Down => print!("v"),
            DirectionalKeypadState::Left => print!("<"),
            DirectionalKeypadState::Right => print!(">"),
        }
    }
    println!();
}

fn solve_part_1(s: &str) -> usize {
    let mut total_complexity = 0;
    for line in s.lines() {
        let input = parse_numeric(line);
        let code = get_numeric_code(line);
        let shortest_len = shortest_len::<2>(&input);
        total_complexity += shortest_len * code
    }
    total_complexity
}

fn solve_part_2(s: &str) -> usize {
    let mut total_complexity = 0;
    for line in s.lines() {
        println!("Running");
        let input = parse_numeric(line);
        let code = get_numeric_code(line);
        let shortest_len = shortest_len::<10>(&input);
        total_complexity += shortest_len * code
    }
    total_complexity
}

pub(crate) fn part_1(input: String) {
    println!("Total complexity is {}", solve_part_1(&input));
}

pub(crate) fn part_2(input: String) {
    println!("Total complexity is {}", solve_part_2(&input));
}

fn shortest_len<const N: usize>(codes: &[NumericKeypadState]) -> usize {
    let set = generic_dfs_nohistory(
        State::<N>::default(),
        |state, _| state.sequence.as_slice() == codes,
        |state, _| !codes.starts_with(state.sequence.as_slice()),
        State::get_neighbours,
        &(),
    );
    let (f, w) = set
        .into_iter()
        .find(|(s, _)| s.sequence.as_slice() == codes)
        .unwrap();
    w
}

#[test]
fn test_part_1() {
    let input = "029A
980A
179A
456A
379A";
    assert_eq!(solve_part_1(input), 126384);
}
#[test]
fn test_part_1_shortest_1() {
    let test_in = [
        NumericKeypadState::Zero,
        NumericKeypadState::Two,
        NumericKeypadState::Nine,
        NumericKeypadState::A,
    ];
    assert_eq!(shortest_len::<2>(&test_in), 68)
}
#[test]
fn test_expected_state() {
    let seq =
        parse_directional("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
    let state = State::<2>::default();
    let state = state.test_seq(seq).unwrap();
    assert_eq!(
        state.sequence,
        vec![
            NumericKeypadState::Zero,
            NumericKeypadState::Two,
            NumericKeypadState::Nine,
            NumericKeypadState::A,
        ]
    );
}
