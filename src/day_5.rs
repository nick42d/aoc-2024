use std::{cmp::Ordering, collections::HashMap};

enum OrderingRule {
    Before(usize),
    After(usize),
}

fn parse_input(s: &str) -> (HashMap<usize, Vec<OrderingRule>>, Vec<Vec<usize>>) {
    let (rules, updates) = s.split_once("\n\n").unwrap();
    let updates = updates
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|page_no| str::parse(page_no).unwrap())
                .collect()
        })
        .collect();
    let rules = rules.lines().map(|rule| {
        let (before, after) = rule.split_once("|").unwrap();
        (before.parse().unwrap(), after.parse().unwrap())
    });
    let mut rules_map = HashMap::new();
    for (before, after) in rules {
        rules_map
            .entry(before)
            .and_modify(|e: &mut Vec<OrderingRule>| e.push(OrderingRule::Before(after)))
            .or_insert(vec![OrderingRule::Before(after)]);
        rules_map
            .entry(after)
            .and_modify(|e: &mut Vec<OrderingRule>| e.push(OrderingRule::After(before)))
            .or_insert(vec![OrderingRule::After(before)]);
    }
    (rules_map, updates)
}

// Returns middle nunmber of update if it meets all the rules or 0 otherwise.
fn rules_check(rules: &HashMap<usize, Vec<OrderingRule>>, update: Vec<usize>) -> usize {
    if update.is_sorted_by(|a, b| ordering(a, b, rules).is_le()) {
        return update[update.len() / 2];
    }
    0
}

// If an update is incorrect, re-order it and return the middle number.
// Otherwise, return 0.
fn fix_incorrect_update(
    rules: &HashMap<usize, Vec<OrderingRule>>,
    mut update: Vec<usize>,
) -> usize {
    let mut is_incorrect = false;
    if !update.is_sorted_by(|a, b| ordering(a, b, rules).is_le()) {
        is_incorrect = true;
        update.sort_by(|a, b| ordering(a, b, rules));
    }
    if is_incorrect {
        return update[update.len() / 2];
    }
    0
}

fn ordering(a: &usize, b: &usize, rules: &HashMap<usize, Vec<OrderingRule>>) -> Ordering {
    let Some(rules) = rules.get(a) else {
        return Ordering::Equal;
    };
    for rule in rules {
        match rule {
            OrderingRule::Before(n) if b == n => {
                return Ordering::Less;
            }
            OrderingRule::After(n) if b == n => {
                return Ordering::Greater;
            }
            _ => (),
        }
    }
    Ordering::Equal
}

pub(crate) fn part_1(input: String) {
    let (rules, updates) = parse_input(&input);
    let mut total = 0;
    for update in updates {
        total += rules_check(&rules, update);
    }
    println!("{total}");
}

pub(crate) fn part_2(input: String) {
    let (rules, updates) = parse_input(&input);
    let mut total = 0;
    for update in updates {
        total += fix_incorrect_update(&rules, update);
    }
    println!("{total}");
}
