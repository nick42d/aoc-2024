use crate::utils::Direction;
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

#[derive(Hash, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    sequence: Vec<NumericKeypadState>,
    // Robot 0 has controls to numeric keypad
    robot_0: NumericKeypadState,
    // Robot 1 has controls to Robot 0 directional keypad
    robot_1: DirectionalKeypadState,
    // Robot 2 has controls to Robot 1 directional keypad
    // You have controls to Robot 2 directional keypad.
    robot_2: DirectionalKeypadState,
}

impl State {
    fn get_neighbours(self) -> impl Iterator<Item = (Self, DirectionalKeypadState)> {
        let seq_len = self.sequence.len();
        [
            DirectionalKeypadState::A,
            DirectionalKeypadState::Up,
            DirectionalKeypadState::Down,
            DirectionalKeypadState::Left,
            DirectionalKeypadState::Right,
        ]
        .into_iter()
        .filter_map(move |d| Some((self.clone().press_directional(d)?, d)))
    }
    fn press_directional(mut self, d: DirectionalKeypadState) -> Option<Self> {
        match d {
            DirectionalKeypadState::A => match self.robot_2 {
                DirectionalKeypadState::A => match self.robot_1 {
                    DirectionalKeypadState::A => {
                        self.sequence.push(self.robot_0);
                        Some(self)
                    }
                    dir => {
                        self.robot_0 = self.robot_0.move_arm(dir.get_dir_unchecked())?;
                        Some(self)
                    }
                },
                dir => {
                    self.robot_1 = self.robot_1.move_arm(dir.get_dir_unchecked())?;
                    Some(self)
                }
            },
            DirectionalKeypadState::Up => {
                let new_robot_2 = self.robot_2.move_arm(Direction::Up)?;
                let mut this = self.clone();
                this.robot_2 = new_robot_2;
                Some(this)
            }
            DirectionalKeypadState::Down => {
                let new_robot_2 = self.robot_2.move_arm(Direction::Down)?;
                let mut this = self.clone();
                this.robot_2 = new_robot_2;
                Some(this)
            }
            DirectionalKeypadState::Left => {
                let new_robot_2 = self.robot_2.move_arm(Direction::Left)?;
                let mut this = self.clone();
                this.robot_2 = new_robot_2;
                Some(this)
            }
            DirectionalKeypadState::Right => {
                let new_robot_2 = self.robot_2.move_arm(Direction::Right)?;
                let mut this = self.clone();
                this.robot_2 = new_robot_2;
                Some(this)
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
            (NumericKeypadState::Three, _) => None,
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

pub(crate) fn part_1(input: String) {
    todo!()
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[test]
fn test_part_1_shortest_1() {
    let test_in = [
        NumericKeypadState::Zero,
        NumericKeypadState::Two,
        NumericKeypadState::Nine,
        NumericKeypadState::A,
    ];
    let set = generic_bfs(
        State::default(),
        |state| state.sequence.as_slice() == test_in.as_slice(),
        |state| !test_in.as_slice().starts_with(state.sequence.as_slice()),
        State::get_neighbours,
    );
    let (f, d) = set
        .into_iter()
        .find(|(s, _)| s.sequence.as_slice() == test_in.as_slice())
        .unwrap();
    print_dirs(&d);
    assert_eq!(d.len(), 68)
}
#[test]
fn test_part_1_shortest_2() {
    let test_in = [
        NumericKeypadState::Nine,
        NumericKeypadState::Eight,
        NumericKeypadState::Zero,
        NumericKeypadState::A,
    ];
    let set = generic_bfs(
        State::default(),
        |state| state.sequence.as_slice() == test_in.as_slice(),
        |state| !test_in.as_slice().starts_with(state.sequence.as_slice()),
        State::get_neighbours,
    );
    let (f, d) = set
        .into_iter()
        .find(|(s, _)| s.sequence.as_slice() == test_in.as_slice())
        .unwrap();
    print_dirs(&d);
    assert_eq!(d.len(), 60)
}
#[test]
fn test_part_1_shortest_3() {
    let test_in = [
        NumericKeypadState::One,
        NumericKeypadState::Seven,
        NumericKeypadState::Nine,
        NumericKeypadState::A,
    ];
    let set = generic_bfs(
        State::default(),
        |state| state.sequence.as_slice() == test_in.as_slice(),
        |state| !test_in.as_slice().starts_with(state.sequence.as_slice()),
        State::get_neighbours,
    );
    let (f, d) = set
        .into_iter()
        .find(|(s, _)| s.sequence.as_slice() == test_in.as_slice())
        .unwrap();
    print_dirs(&d);
    assert_eq!(d.len(), 68)
}
#[test]
fn test_part_1_shortest_4() {
    let test_in = [
        NumericKeypadState::Three,
        NumericKeypadState::Seven,
        NumericKeypadState::Nine,
        NumericKeypadState::A,
    ];
    let set = generic_bfs(
        State::default(),
        |state| state.sequence.as_slice() == test_in.as_slice(),
        |state| !test_in.as_slice().starts_with(state.sequence.as_slice()),
        State::get_neighbours,
    );
    let (f, d) = set
        .into_iter()
        .find(|(s, _)| s.sequence.as_slice() == test_in.as_slice())
        .unwrap();
    print_dirs(&d);
    assert_eq!(d.len(), 64)
}
