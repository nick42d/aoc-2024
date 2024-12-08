use std::collections::HashSet;

struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

fn parse(s: &str) -> Vec<Equation> {
    s.lines()
        .map(|line| {
            let (test_value, numbers) = line.split_once(":").unwrap();
            let test_value = str::parse(test_value).unwrap();
            let numbers = numbers
                .trim()
                .split(' ')
                .map(|n| str::parse(n).unwrap())
                .collect();
            Equation {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn concaternate(a: usize, b: usize) -> usize {
    str::parse(&format!("{a}{b}")).unwrap()
}

// Returns 0 if equation did not pass.
fn test_equation(equation: &Equation) -> usize {
    let mut numbers = equation.numbers.iter();
    let first_num = numbers.next().unwrap();
    let tries = numbers.fold(HashSet::from([*first_num]), |acc, e| {
        acc.into_iter().flat_map(|x| [x + e, x * e]).collect()
    });
    if tries.contains(&equation.test_value) {
        return equation.test_value;
    }
    0
}

// Returns 0 if equation did not pass.
fn test_equation_with_concaternation(equation: &Equation) -> usize {
    let mut numbers = equation.numbers.iter();
    let first_num = numbers.next().unwrap();
    let tries = numbers.fold(HashSet::from([*first_num]), |acc, e| {
        acc.into_iter()
            .flat_map(|x| [x + e, x * e, concaternate(x, *e)])
            .collect()
    });
    if tries.contains(&equation.test_value) {
        return equation.test_value;
    }
    0
}

pub(crate) fn part_1(input: String) {
    let mut total = 0;
    for equation in parse(&input) {
        total += test_equation(&equation);
    }
    println!("Total of all passed equations is {total}.");
}

pub(crate) fn part_2(input: String) {
    let mut total = 0;
    for equation in parse(&input) {
        total += test_equation_with_concaternation(&equation);
    }
    println!("Total of all passed equations is {total}.");
}

#[cfg(test)]
mod tests {
    use crate::day_07::{parse, test_equation, test_equation_with_concaternation};

    const TEST_DATA: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    #[test]
    fn part_1() {
        let input = parse(TEST_DATA);
        let mut total = 0;
        for equation in input {
            total += test_equation(&equation)
        }
        assert_eq!(total, 3749);
    }
    #[test]
    fn part_2() {
        let input = parse(TEST_DATA);
        let mut total = 0;
        for equation in input {
            total += test_equation_with_concaternation(&equation)
        }
        assert_eq!(total, 11387);
    }
}
