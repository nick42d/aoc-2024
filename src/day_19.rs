use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet, VecDeque};

/// (patterns, designs). Note, patterns will be sorted.
fn parse_input(s: &str) -> (BTreeSet<String>, Vec<String>) {
    let (patterns, designs) = s.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").map(ToString::to_string).collect();
    let designs = designs.lines().map(ToString::to_string).collect();
    (patterns, designs)
}

fn compress_patterns(patterns: &BTreeSet<String>) -> BTreeMap<String, HashSet<(Vec<String>)>> {
    let mut map = BTreeMap::new();
    let patterns_clone = patterns.clone();
    for p in patterns_clone {
        let p_clone = p.clone();
        map.insert(p_clone, check_design(p, patterns));
    }
    map
}

fn check_design(d: String, patterns: &BTreeSet<String>) -> HashSet<(Vec<String>)> {
    println!("Checking design {d}");
    let d_len = d.len();
    let mut queue = VecDeque::from([(vec![])]);
    let mut tried = HashSet::new();
    loop {
        let Some(mut trail) = queue.pop_front() else {
            break;
        };
        let trail_string = trail
            .iter()
            .map(|s: &String| s.as_str())
            .collect::<String>();
        if trail_string == d {
            // println!("Found matching trail");
            tried.insert(trail);
            continue;
        }
        if trail_string.len() > d.len() {
            // println!("This shouldn't be possible");
            continue;
        }
        // eprintln!("Trail_string: {trail_string}, design {d}");
        let range_start = d.as_str()[trail_string.len()..=trail_string.len()].to_string();
        let range_end = range_start
            .chars()
            .next()
            .map(|c| ((c as u8 + 1) as char).to_string())
            .unwrap();
        for p in patterns.range(range_start..range_end) {
            if d[trail_string.len()..].starts_with(p) {
                // println!("Pattern {p} - match");
                let mut trail_branch = trail.clone();
                trail_branch.push(p.clone());
                queue.push_back(trail_branch.clone());
                tried.insert(trail_branch);
            } else {
                // println!("Pattern {p} - no match");
                // Since patterns are sorted, if we don't find a match, then none of the rest
                // match either.
                continue;
            }
        }
    }
    println!("{:?}", tried);
    tried
}

fn solve_part_1(s: &str) -> usize {
    let (patterns, designs) = parse_input(s);
    let d_len = designs.len();
    let mut total = 0;
    for (i, d) in designs.iter().enumerate() {
        println!("Checking design {i} of {d_len}");
        if check_design(d.clone(), &patterns)
            .iter()
            .any(|trail| &trail.iter().map(|x| x.as_str()).collect::<String>() == d)
        {
            total += 1;
        }
    }
    total
}

fn solve_part_2(s: &str) -> usize {
    let (patterns, designs) = parse_input(s);
    let d_len = designs.len();
    let mut total = 0;
    for d in designs {
        total += check_design(d.clone(), &patterns)
            .iter()
            .filter(|trail| trail.iter().map(|x| x.as_str()).collect::<String>() == d)
            .count()
    }
    total
}

pub(crate) fn part_1(input: String) {
    println!("Total designs possible: {}", solve_part_1(&input));
}

pub(crate) fn part_2(input: String) {
    let (patterns, designs) = parse_input(&input);
    let compressed = compress_patterns(&patterns);
    for (p, ps) in compressed {
        println!("{p}, {}", ps.len());
    }
    // println!("Total designs possible: {}", solve_part_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::day_19::{solve_part_1, solve_part_2};

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
    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TEST_DATA), 16);
    }
}
