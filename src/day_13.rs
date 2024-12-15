use std::cmp::Ordering;

const A_COST: isize = 3;
const B_COST: isize = 1;

#[derive(Debug)]
struct ClawMachine {
    a_x: isize,
    a_y: isize,
    b_x: isize,
    b_y: isize,
    prize_x: isize,
    prize_y: isize,
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
    WinWithA(isize),
    NotWinnable,
}

/// This is the value of n_a and n_b where claw machine wins. Returns (value,
/// remainder)
fn solve(c: &ClawMachine) -> Option<(isize, isize)> {
    let n_b = (c.a_x * c.prize_y - c.a_y * c.prize_x) / (c.b_y * c.a_x - c.b_x * c.a_y);
    let rem_b = (c.a_x * c.prize_y - c.a_y * c.prize_x) % (c.b_y * c.a_x - c.b_x * c.a_y);
    let n_a = (c.prize_x - n_b * c.b_x) / c.a_x;
    let rem_a = (c.prize_x - n_b * c.b_x) % c.a_x;
    if rem_b != 0 || rem_a != 0 {
        return None;
    };
    Some((n_a, n_b))
}

fn solve_part_1(s: &str) -> isize {
    let c = parse_input(s);
    c.into_iter()
        .filter_map(|c| solve(&c))
        .map(|(a, b)| a * A_COST + b * B_COST)
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn solve_part_2(s: &str) -> isize {
    let c = parse_input(s);
    c.into_iter()
        .map(|mut c| {
            c.prize_x += 10000000000000;
            c.prize_y += 10000000000000;
            c
        })
        .filter_map(|c| solve(&c))
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
    println!(
        "Minimum tokens to win all possible prizes is {}",
        solve_part_2(&input)
    );
}

#[cfg(test)]
mod tests {
    use crate::day_13::{parse_input, solve, solve_part_1, ClawMachine};

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
        let (a, b) = solve(c).unwrap();
        assert_eq!((a, b), (80, 40));
    }
    #[test]
    fn test_part_1_machine_2() {
        let c = parse_input(TEST_DATA);
        let c = &c[1];
        let w = solve(c);
        assert_eq!(w, None);
    }
    #[test]
    fn test_part_1_machine_3() {
        let c = parse_input(TEST_DATA);
        let c = &c[2];
        let w = solve(c);
        assert_eq!(w, Some((38, 86)));
    }
    #[test]
    fn test_part_1_machine_4() {
        let c = parse_input(TEST_DATA);
        let c = &c[3];
        let w = solve(c);
        assert_eq!(w, None);
    }
}
