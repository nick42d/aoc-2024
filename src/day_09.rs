use std::collections::VecDeque;

#[derive(Debug)]
struct Encoding {
    files: usize,
    free_space: usize,
    idx: usize,
}

#[derive(Debug)]
struct MovableEncoding {
    files: usize,
    free_space: usize,
    idx: usize,
    now_fixed: bool,
}

fn next_compacted_block(encodings: &mut VecDeque<Encoding>) -> Option<usize> {
    let mut front = encodings.pop_front()?;
    if front.files > 0 {
        let next = front.idx;
        front.files -= 1;
        if front.files > 0 || front.free_space > 0 {
            encodings.push_front(front);
        }
        return Some(next);
    }
    let mut back = encodings.pop_back()?;
    if back.files > 0 {
        let next = back.idx;
        front.free_space -= 1;
        back.files -= 1;
        if back.files > 0 {
            encodings.push_back(back);
        }
        if front.free_space > 0 {
            encodings.push_front(front);
        }
        return Some(next);
    }
    unreachable!()
}

fn next_block(encodings: &mut VecDeque<MovableEncoding>) -> Option<usize> {
    let mut front = encodings.pop_front()?;
    let mut next = 0;
    if front.files > 0 {
        next = front.idx;
        front.files -= 1;
    } else if front.free_space > 0 {
        front.free_space -= 1;
    }
    if front.files > 0 || front.free_space > 0 {
        encodings.push_front(front);
    }
    Some(next)
}

fn parse_encodings(s: &str) -> VecDeque<Encoding> {
    let trimmed = s.trim();
    trimmed
        .chars()
        .step_by(2)
        .zip(
            trimmed
                .chars()
                .chain(std::iter::once('0'))
                .skip(1)
                .step_by(2),
        )
        .enumerate()
        .map(|(idx, (files, free_space))| Encoding {
            files: files.to_digit(10).unwrap() as usize,
            free_space: free_space.to_digit(10).unwrap() as usize,
            idx,
        })
        .collect()
}

fn parse_movable_encodings(s: &str) -> VecDeque<MovableEncoding> {
    let trimmed = s.trim();
    trimmed
        .chars()
        .step_by(2)
        .zip(
            trimmed
                .chars()
                .chain(std::iter::once('0'))
                .skip(1)
                .step_by(2),
        )
        .enumerate()
        .map(|(idx, (files, free_space))| MovableEncoding {
            files: files.to_digit(10).unwrap() as usize,
            free_space: free_space.to_digit(10).unwrap() as usize,
            idx,
            now_fixed: false,
        })
        .collect()
}

fn defrag(encodings: &mut VecDeque<MovableEncoding>) {
    let mut back_idx = encodings.len();
    while back_idx != 0 {
        back_idx -= 1;
        if !encodings[back_idx].now_fixed {
            encodings[back_idx].now_fixed = true;
            let trying_to_move_files = encodings[back_idx].files;
            for fwd_idx in 0..back_idx {
                if let Some(space) = encodings[fwd_idx]
                    .free_space
                    .checked_sub(trying_to_move_files)
                {
                    let new_gap = encodings[back_idx].free_space + encodings[back_idx].files;
                    encodings[fwd_idx].free_space = 0;
                    encodings[back_idx].free_space = space;
                    if let Some(next_back_idx) = back_idx.checked_sub(1) {
                        debug_assert_ne!(next_back_idx, fwd_idx);
                        encodings[next_back_idx].free_space += new_gap
                    }
                    let back = encodings.remove(back_idx).unwrap();
                    encodings.insert(fwd_idx + 1, back);
                    // Neat hack to say, need to try this back_idx a second time if we've done a
                    // swap, since it will contain a new value.
                    back_idx += 1;
                    break;
                }
            }
        }
    }
    debug_assert!(back_idx <= 1)
}

fn print_encodings(encodings: &VecDeque<MovableEncoding>) {
    for encoding in encodings {
        for file in 0..encoding.files {
            print!("|{}", encoding.idx);
        }
        for space in 0..encoding.free_space {
            print!(".");
        }
    }
}

pub(crate) fn part_1(input: String) {
    let mut encodings = parse_encodings(&input);
    let mut output = 0;
    let mut counter = 0;
    while let Some(block) = next_compacted_block(&mut encodings) {
        output += block * counter;
        counter += 1;
    }
    println!("Filesystem checksum is {output}");
}

pub(crate) fn part_2(input: String) {
    let mut encodings = parse_movable_encodings(&input);
    let mut output = 0;
    let mut counter = 0;
    defrag(&mut encodings);
    while let Some(block) = next_block(&mut encodings) {
        output += block * counter;
        counter += 1;
    }
    println!("Filesystem checksum is {output}");
}

#[cfg(test)]
mod tests {
    use super::parse_encodings;
    use crate::day_09::{defrag, next_block, next_compacted_block, parse_movable_encodings};

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        let mut encodings = parse_encodings(TEST_INPUT);
        let mut output = 0;
        let mut counter = 0;
        while let Some(block) = next_compacted_block(&mut encodings) {
            output += block * counter;
            counter += 1;
        }
        assert_eq!(output, 1928);
    }

    #[test]
    fn test_part_2() {
        let mut encodings = parse_movable_encodings(TEST_INPUT);
        let mut output = 0;
        let mut counter = 0;
        defrag(&mut encodings);
        while let Some(block) = next_block(&mut encodings) {
            output += block * counter;
            counter += 1;
        }
        assert_eq!(output, 2858);
    }
}
