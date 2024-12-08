use std::{cmp::Ordering, ops::ControlFlow};

#[derive(Debug)]
enum State {
    Init,
    Iteration1 { prev: usize },
    Iteration2 { prev: usize, ordering: Ordering },
    Unsafe,
}

fn list_is_safe(list: &[usize]) -> bool {
    let safe = list.iter().try_fold(State::Init, |state, &e| match state {
        State::Init => ControlFlow::Continue(State::Iteration1 { prev: e }),
        State::Iteration1 { prev } => {
            let ordering = e.cmp(&prev);
            if ordering == Ordering::Equal || e.abs_diff(prev) > 3 {
                return ControlFlow::Break(State::Unsafe);
            }
            ControlFlow::Continue(State::Iteration2 { prev: e, ordering })
        }
        State::Iteration2 { prev, ordering } => {
            let new_ordering = e.cmp(&prev);
            if new_ordering == Ordering::Equal || new_ordering != ordering || e.abs_diff(prev) > 3 {
                return ControlFlow::Break(State::Unsafe);
            }
            ControlFlow::Continue(State::Iteration2 { prev: e, ordering })
        }
        State::Unsafe => unreachable!(),
    });
    matches!(safe, ControlFlow::Continue(_))
}

fn parse_list(list: &str) -> Vec<usize> {
    list.split(" ").map(|x| str::parse(x).unwrap()).collect()
}

pub fn part_1(file: String) {
    let safety = file
        .lines()
        .map(parse_list)
        .map(|list| list_is_safe(&list))
        .fold(0, |mut acc, e| {
            if e {
                acc += 1
            };
            acc
        });
    println!("list is safe: {safety}");
}

pub fn part_2(file: String) {
    // Brute force solution
    let safety = file
        .lines()
        .map(parse_list)
        .map(|list| {
            if list_is_safe(&list) {
                return true;
            }
            for i in 0..list.len() {
                let mut tmp = list.clone();
                tmp.remove(i);
                if list_is_safe(&tmp) {
                    return true;
                }
            }
            false
        })
        .fold(0, |mut acc, e| {
            if e {
                acc += 1
            };
            acc
        });
    println!("list is safe: {safety}");
}
