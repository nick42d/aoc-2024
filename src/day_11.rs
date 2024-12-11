use std::collections::HashMap;

fn input_to_vec(s: &str) -> Vec<usize> {
    s.trim()
        .split(' ')
        .map(|n| str::parse(n).unwrap())
        .collect()
}

fn input_to_map(s: &str) -> HashMap<usize, usize> {
    s.trim()
        .split(' ')
        .map(|n| (str::parse(n).unwrap(), 1))
        .collect()
}

fn get_digits(n: usize) -> u32 {
    n.ilog10() + 1
}

fn split_int(n: usize) -> (usize, usize) {
    let digits = get_digits(n);
    let n1 = n / 10usize.pow(digits / 2);
    let n2 = n - n1 * 10usize.pow(digits / 2);
    (n1, n2)
}

fn next_stones(n: usize) -> (usize, Option<usize>) {
    match n {
        0 => (1, None),
        other if get_digits(other) % 2 == 0 => {
            let (n1, n2) = split_int(n);
            (n1, Some(n2))
        }
        other => (other * 2024, None),
    }
}

fn blink_all(stones: Vec<usize>) -> Vec<usize> {
    let mut next_state = vec![];
    for stone in stones {
        let (s1, maybe_s2) = next_stones(stone);
        next_state.push(s1);
        if let Some(s2) = maybe_s2 {
            next_state.push(s2);
        }
    }
    next_state
}

fn blink_all_compressed(stones: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut next_state = HashMap::new();
    for (stone, count) in stones {
        let (s1, maybe_s2) = next_stones(stone);
        next_state
            .entry(s1)
            .and_modify(|new_count| *new_count += count)
            .or_insert(count);
        if let Some(s2) = maybe_s2 {
            next_state
                .entry(s2)
                .and_modify(|new_count| *new_count += count)
                .or_insert(count);
        }
    }
    next_state
}

pub(crate) fn part_1(input: String) {
    let mut stones = input_to_vec(&input);
    for i in 0..25 {
        stones = blink_all(stones);
    }
    println!("After blinking 25 times, {} stones.", stones.len());
}

pub(crate) fn part_2(input: String) {
    let mut stones = input_to_map(&input);
    for i in 0..75 {
        stones = blink_all_compressed(stones);
    }
    println!(
        "After blinking 75 times, {} stones.",
        stones.into_values().reduce(|acc, e| acc + e).unwrap()
    );
}
