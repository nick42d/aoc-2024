const PINS: usize = 5;
const PIN_HEIGHT: usize = 7;

/// (locks, keys)
fn parse_input(s: &str) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut locks = vec![];
    let mut keys = vec![];
    for pattern in s.split("\n\n") {
        let mut is_lock = pattern.lines().next().unwrap().starts_with('#');
        let mut schematic = [99; PINS];
        for (y, col) in schematic.iter_mut().enumerate() {
            for row in 0..PIN_HEIGHT {
                match is_lock {
                    true => {
                        if pattern.lines().nth(row).unwrap().chars().nth(y).unwrap() == '.' {
                            *col = row - 1;
                            break;
                        }
                    }
                    false => {
                        if pattern.lines().nth(row).unwrap().chars().nth(y).unwrap() == '#' {
                            *col = PIN_HEIGHT - row - 1;
                            break;
                        }
                    }
                }
            }
        }
        debug_assert!(!schematic.contains(&99));
        match is_lock {
            true => locks.push(schematic),
            false => keys.push(schematic),
        }
    }
    (locks, keys)
}

fn lock_fits_key(lock: &[usize], key: &[usize]) -> bool {
    // True if none of the pin totals greater than pin height.
    !lock
        .iter()
        .zip(key.iter())
        .map(|(l, k)| l + k)
        .any(|total| total >= PIN_HEIGHT - 1)
}

fn get_number_of_fits(locks: &[[usize; 5]], keys: &[[usize; 5]]) -> usize {
    let mut total = 0;
    for lock in locks {
        for key in keys {
            if lock_fits_key(lock, key) {
                total += 1;
            }
        }
    }
    total
}

fn solve_part_1(s: &str) -> usize {
    let (locks, keys) = parse_input(s);
    get_number_of_fits(&locks, &keys)
}

pub(crate) fn part_1(input: String) {
    println!(
        "Total unique lock key pairs that fit: {}",
        solve_part_1(&input)
    );
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[test]
fn test_parse() {
    let (locks, keys) = parse_input(TEST_DATA);
    assert_eq!(locks[0], [0, 5, 3, 4, 3]);
    assert_eq!(keys[0], [5, 0, 2, 1, 3]);
}

#[test]
fn test_fits() {
    assert!(!lock_fits_key(&[0, 5, 3, 4, 3], &[5, 0, 2, 1, 3]));
    assert!(!lock_fits_key(&[0, 5, 3, 4, 3], &[4, 3, 4, 0, 2]));
    assert!(lock_fits_key(&[0, 5, 3, 4, 3], &[3, 0, 2, 0, 1]));
}

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(TEST_DATA), 3);
}

const TEST_DATA: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
