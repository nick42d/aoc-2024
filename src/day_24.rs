use std::{
    arch::x86_64::_CMP_EQ_US,
    collections::{BTreeMap, HashMap},
};

pub(crate) fn part_1(input: String) {
    println!("Decimal number is {}", solve_part_1(&input));
}

pub(crate) fn part_2(input: String) {
    todo!()
}

fn solve_part_1(s: &str) -> usize {
    let (vals, eqs) = parse_input(s);
    let vals = get_all_vals(vals, eqs);
    vals.into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(_, v)| v as usize)
        .enumerate()
        .fold(0, |acc, (idx, e)| acc + e * 2usize.pow(idx as u32))
}

struct Eq {
    a: Expr,
    b: Expr,
    val: String,
    op: Op,
}

enum Op {
    And,
    Xor,
    Or,
}

impl Op {
    fn eval(&self, a: bool, b: bool) -> bool {
        match self {
            Op::And => a & b,
            Op::Xor => a ^ b,
            Op::Or => a | b,
        }
    }
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!(),
        }
    }
}

enum Expr {
    Val(bool),
    Var(String),
}

/// (vals, eqs)
fn parse_input(s: &str) -> (BTreeMap<String, bool>, Vec<Eq>) {
    let (vals, eqs) = s.split_once("\n\n").unwrap();
    let vals = vals
        .lines()
        .map(|line| {
            let (var, val) = line.split_once(": ").unwrap();
            (var.to_string(), val.parse::<u8>().unwrap() != 0)
        })
        .collect();
    let eqs = eqs
        .lines()
        .map(|line| {
            let (iter, val) = line.split_once(" -> ").unwrap();
            let val = val.to_string();
            let mut iter = iter.split(" ");
            let a = Expr::Var(iter.next().unwrap().to_string());
            let op = Op::from(iter.next().unwrap());
            let b = Expr::Var(iter.next().unwrap().to_string());
            Eq { a, b, val, op }
        })
        .collect();
    (vals, eqs)
}

fn get_all_vals(mut vals: BTreeMap<String, bool>, mut eqs: Vec<Eq>) -> BTreeMap<String, bool> {
    while !eqs.is_empty() {
        let mut neweqs = Vec::new();
        for mut eq in eqs {
            let Eq {
                mut a,
                mut b,
                val,
                op,
            } = eq;
            if let Expr::Var(ref var) = a {
                if let Some(val) = vals.get(var) {
                    a = Expr::Val(*val)
                }
            }
            if let Expr::Var(ref var) = b {
                if let Some(val) = vals.get(var) {
                    b = Expr::Val(*val)
                }
            }
            if let (Expr::Val(a), Expr::Val(b)) = (&a, &b) {
                vals.insert(val, op.eval(*a, *b));
            } else {
                neweqs.push(Eq { a, b, val, op });
            }
        }
        eqs = neweqs;
    }
    vals
}

#[test]
fn test_part_1_1() {
    assert_eq!(solve_part_1(TEST_DATA_1), 4)
}

#[test]
fn test_part_1_2() {
    assert_eq!(solve_part_1(TEST_DATA_2), 2024)
}

const TEST_DATA_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

const TEST_DATA_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
