use std::cmp::Ordering;

const A_COST: usize = 3;
const B_COST: usize = 1;

#[derive(Debug)]
struct ClawMachine {
    a_x: usize,
    a_y: usize,
    b_x: usize,
    b_y: usize,
    prize_x: usize,
    prize_y: usize,
}

fn parse_input(s: &str) -> Vec<ClawMachine> {
    s.split("\n\n")
        .map(|m| {
            let mut lines = m.lines().map(|line| line.split_once(", ").unwrap());
            let (a_x, a_y) = lines.next().unwrap();
            let a_x = a_x.trim_start_matches("Button A: X+").parse().unwrap();
            let a_y = a_y.trim_start_matches("Y+").parse().unwrap();
            let (b_x, b_y) = lines.next().unwrap();
            let b_x = b_x.trim_start_matches("Button B: X+").parse().unwrap();
            let b_y = b_y.trim_start_matches("Y+").parse().unwrap();
            let (prize_x, prize_y) = lines.next().unwrap();
            let prize_x = prize_x.trim_start_matches("Prize: X=").parse().unwrap();
            let prize_y = prize_y.trim_start_matches("Y=").parse().unwrap();
            ClawMachine {
                a_x,
                a_y,
                b_x,
                b_y,
                prize_x,
                prize_y,
            }
        })
        .collect()
}

#[derive(Debug)]
enum CheckOutcome {
    TooMuchB,
    NotEnoughB,
    WinWithA(usize),
    NotWinnable,
}

// Returns if we can win with the current n_b, or if more or less n_b are
// required. If indivisible, return None.
fn check_win(c: &ClawMachine, n_b: usize) -> CheckOutcome {
    println!("c: {:?}", c);
    let b_x = c.b_x * n_b;
    let b_y = c.b_y * n_b;
    let Some(x_rem) = c.prize_x.checked_sub(b_x) else {
        return CheckOutcome::TooMuchB;
    };
    let Some(y_rem) = c.prize_y.checked_sub(b_y) else {
        return CheckOutcome::TooMuchB;
    };

    let n_a_x = x_rem / c.a_x;
    let n_a_y = y_rem / c.a_y;
    let n_a_x_rem = x_rem % c.a_x;
    let n_a_y_rem = y_rem % c.a_y;

    if n_a_x == n_a_y {
        if n_a_y_rem == 0 && n_a_x_rem == 0 {
            return CheckOutcome::WinWithA(n_a_x);
        }
        return CheckOutcome::NotWinnable;
    }
    CheckOutcome::NotEnoughB
}

/// If b is pressed n_b times, returns number of times a pressed if it will be a
/// win.
fn n_a_to_win(c: &ClawMachine, n_b: usize) -> Option<usize> {
    let b_x = c.b_x * n_b;
    let b_y = c.b_y * n_b;
    let x_rem = c.prize_x.checked_sub(b_x)?;
    let y_rem = c.prize_y.checked_sub(b_y)?;

    if x_rem / c.a_x == y_rem / c.a_y && x_rem % c.a_x == 0 && y_rem % c.a_y == 0 {
        return Some(x_rem / c.a_x);
    }
    None
}

/// If a is pressed n_a times, returns number of times b pressed if it will be a
/// win.
fn n_b_to_win(c: &ClawMachine, n_a: usize) -> Option<usize> {
    let a_x = c.a_x * n_a;
    let a_y = c.a_y * n_a;
    let x_rem = c.prize_x.checked_sub(a_x)?;
    let y_rem = c.prize_y.checked_sub(a_y)?;

    if x_rem / c.b_x == y_rem / c.b_y && x_rem % c.b_x == 0 && y_rem % c.b_y == 0 {
        return Some(x_rem / c.b_x);
    }
    None
}

/// (a, b)
fn cheapest_win(c: &ClawMachine) -> Option<(usize, usize)> {
    let mut cur = 0;
    let mut max_bounds = c.prize_x.max(c.prize_y);
    let mut min_bounds = 0;
    loop {
        println!("mx {max_bounds} mn {min_bounds} cur {cur}");
        let check = check_win(c, cur);
        println!("Check: {:?}", check);
        match check {
            CheckOutcome::NotEnoughB => {
                min_bounds = cur;
                cur = (max_bounds + cur) / 2;
            }
            CheckOutcome::TooMuchB => {
                max_bounds = cur;
                cur = (min_bounds + cur) / 2;
            }
            CheckOutcome::WinWithA(n) => return Some((n, cur)),
            CheckOutcome::NotWinnable => return None,
        }
    }
}

fn solve_part_1(s: &str) -> usize {
    let c = parse_input(s);
    c.into_iter()
        .filter_map(|c| cheapest_win(&c))
        .map(|(a, b)| a * A_COST + b * B_COST)
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn solve_part_2(s: &str) -> usize {
    let c = parse_input(s);
    c.into_iter()
        .map(|mut c| {
            c.prize_x += 10000000000000;
            c.prize_y += 10000000000000;
            c
        })
        .enumerate()
        .map(|(idx, e)| {
            println!("checking machine {idx}");
            e
        })
        .filter_map(|c| cheapest_win(&c))
        .map(|(a, b)| a * A_COST + b * B_COST)
        .reduce(|acc, e| acc + e)
        .unwrap()
}

pub(crate) fn part_1(input: String) {
    println!(
        "Minimum tokens to win all possible prizes is {}",
        solve_part_1(&input)
    );
}

pub(crate) fn part_2(input: String) {
    // println!(
    //     "Minimum tokens to win all possible prizes is {}",
    //     solve_part_2(&input)
    // );
}

#[cfg(test)]
mod tests {
    use crate::day_13::{cheapest_win, parse_input, solve_part_1, ClawMachine};

    const TEST_DATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TEST_DATA), 480);
    }
    #[test]
    fn test_part_1_machine_1() {
        let c = parse_input(TEST_DATA);
        let c = &c[0];
        let (a, b) = cheapest_win(c).unwrap();
        assert_eq!((a, b), (80, 40));
    }
    #[test]
    fn test_part_1_machine_2() {
        let c = parse_input(TEST_DATA);
        let c = &c[1];
        let w = cheapest_win(c);
        assert_eq!(w, None);
    }
    #[test]
    fn test_part_1_machine_3() {
        let c = parse_input(TEST_DATA);
        let c = &c[2];
        let w = cheapest_win(c);
        assert_eq!(w, Some((38, 86)));
    }
    #[test]
    fn test_part_1_machine_4() {
        let c = parse_input(TEST_DATA);
        let c = &c[3];
        let w = cheapest_win(c);
        assert_eq!(w, None);
    }
}
