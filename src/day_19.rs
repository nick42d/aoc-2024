use std::collections::{BinaryHeap, HashSet};

/// (patterns, designs)
fn parse_input(s: &str) -> (Vec<String>, Vec<String>) {
    let (patterns, designs) = s.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").map(ToString::to_string).collect();
    let designs = designs.lines().map(ToString::to_string).collect();
    (patterns, designs)
}

fn check_design(d: String, patterns: &[String]) -> bool {
    let d_len = d.len();
    let mut cur = 0;
    let mut queue = BinaryHeap::from([0]);
    let mut tried = HashSet::new();
    let mut success = false;
    loop {
        let Some(next_try) = queue.pop() else { break };
        if next_try == d_len {
            success = true;
            break;
        }
        for p in patterns {
            if d[next_try..].starts_with(p) {
                queue.push(next_try + p.len());
                tried.insert(next_try + p.len());
            }
        }
    }
    success
}

fn solve_part_1(s: &str) -> usize {
    let (patterns, designs) = parse_input(s);
    let mut total = 0;
    for d in designs {
        if check_design(d, &patterns) {
            total += 1;
        }
    }
    total
}

pub(crate) fn part_1(input: String) {
    println!("Total designs possible: {}", solve_part_1(&input));
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day_19::solve_part_1;

    const TEST_DATA: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TEST_DATA), 6);
    }
}
