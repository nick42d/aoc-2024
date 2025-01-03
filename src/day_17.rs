use std::{
    ops::Deref,
    sync::{mpsc, Arc, Mutex},
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    ptr: usize,
    out: Vec<usize>,
    ins: Vec<usize>,
}

enum OneOrIter<T, I = std::iter::Once<T>>
where
    I: Iterator<Item = T>,
{
    One(T),
    Iter(I),
}

impl<T, I> IntoIterator for OneOrIter<T, I>
where
    I: Iterator<Item = T> + 'static,
    T: 'static,
{
    type Item = T;
    type IntoIter = Box<dyn Iterator<Item = T>>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            OneOrIter::One(x) => Box::new(std::iter::once(x)),
            OneOrIter::Iter(i) => Box::new(i),
        }
    }
}

impl<T, I> Clone for OneOrIter<T, I>
where
    I: Iterator<Item = T>,
    T: Copy,
    I: Copy,
{
    fn clone(&self) -> OneOrIter<T, I> {
        match self {
            OneOrIter::One(x) => OneOrIter::One(*x),
            OneOrIter::Iter(i) => OneOrIter::Iter(*i),
        }
    }
}

impl<T, I> OneOrIter<T, I>
where
    I: Iterator<Item = T>,
{
    fn map<U, F>(self, mut f: F) -> OneOrIter<U, impl Iterator<Item = U>>
    where
        F: FnMut(T) -> U + Clone + 'static,
        T: 'static,
    {
        match self {
            OneOrIter::One(x) => OneOrIter::One(f(x)),
            OneOrIter::Iter(i) => OneOrIter::Iter(i.map(f)),
        }
    }
    fn and_then<U, F, UI>(self, mut f: F) -> OneOrIter<U, impl Iterator<Item = U>>
    where
        F: FnMut(T) -> OneOrIter<U, UI> + 'static,
        UI: Iterator<Item = U> + 'static,
        I: 'static,
        U: 'static,
        T: 'static,
    {
        match self {
            OneOrIter::One(x) => match f(x) {
                OneOrIter::One(x) => OneOrIter::One(x),
                OneOrIter::Iter(i) => OneOrIter::Iter(Box::new(i) as Box<dyn Iterator<Item = U>>),
            },
            OneOrIter::Iter(i) => {
                let iter = i.map(f).flat_map(|ii| match ii {
                    OneOrIter::One(x) => {
                        Box::new(std::iter::once(x)) as Box<dyn Iterator<Item = U>>
                    }
                    OneOrIter::Iter(i) => Box::new(i) as Box<dyn Iterator<Item = U>>,
                });
                OneOrIter::Iter(Box::new(iter) as Box<dyn Iterator<Item = U>>)
            }
        }
    }
    fn type_erase(self) -> OneOrIter<T, Box<dyn Iterator<Item = T>>>
    where
        T: Copy,
        I: Copy + 'static,
    {
        {
            match self {
                OneOrIter::One(x) => OneOrIter::One(x),
                OneOrIter::Iter(i) => OneOrIter::Iter(Box::new(i)),
            }
        }
    }
}

struct ReversibleComputer<AI, BI, CI>
where
    AI: Iterator<Item = usize>,
    BI: Iterator<Item = usize>,
    CI: Iterator<Item = usize>,
{
    a: OneOrIter<usize, AI>,
    b: OneOrIter<usize, BI>,
    c: OneOrIter<usize, CI>,
    jnzfrm: Option<usize>,
    ptr: usize,
    out: Vec<usize>,
    ins: Vec<usize>,
}

impl<AI, BI, CI> Clone for ReversibleComputer<AI, BI, CI>
where
    AI: Iterator<Item = usize> + Copy,
    BI: Iterator<Item = usize> + Copy,
    CI: Iterator<Item = usize> + Copy,
{
    fn clone(&self) -> Self {
        ReversibleComputer {
            a: self.a.clone(),
            b: self.b.clone(),
            c: self.c.clone(),
            jnzfrm: self.jnzfrm,
            ptr: self.ptr,
            out: self.out.clone(),
            ins: self.ins.clone(),
        }
    }
}

impl<AI, BI, CI> ReversibleComputer<AI, BI, CI>
where
    AI: Iterator<Item = usize> + Copy + 'static,
    BI: Iterator<Item = usize> + Copy + 'static,
    CI: Iterator<Item = usize> + Copy + 'static,
{
    fn from_str(s: &str) -> ReversibleComputer<AI, BI, CI> {
        let mut lines = s.lines();
        let a = lines
            .next()
            .unwrap()
            .trim_start_matches("Register A: ")
            .parse()
            .unwrap();
        let b = lines
            .next()
            .unwrap()
            .trim_start_matches("Register B: ")
            .parse()
            .unwrap();
        let c = lines
            .next()
            .unwrap()
            .trim_start_matches("Register C: ")
            .parse()
            .unwrap();
        let ins = lines
            .nth(1)
            .unwrap()
            .trim_start_matches("Program: ")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        ReversibleComputer {
            a: OneOrIter::One(a),
            b: OneOrIter::One(b),
            c: OneOrIter::One(c),
            ptr: 0,
            out: Vec::new(),
            ins,
            jnzfrm: None,
        }
    }
    fn literal_operand(&self) -> usize {
        self.ins[self.ptr + 1]
    }
    fn literal_operand_back(&self) -> usize {
        self.ins[self.ptr - 1]
    }
    fn combo_operand(&self) -> OneOrIter<usize, impl Iterator<Item = usize>> {
        let combo = self.ins[self.ptr + 1];
        match combo {
            0..=3 => OneOrIter::One(combo),
            4 => self.a.clone().type_erase(),
            5 => self.b.clone().type_erase(),
            6 => self.c.clone().type_erase(),
            7 => unreachable!(),
            _ => panic!(),
        }
    }
    fn cur(&self) -> Option<usize> {
        self.ins.get(self.ptr).copied()
    }
    fn cur_back(&self) -> Option<usize> {
        self.ins.get(self.ptr - 2).copied()
    }
    fn inc_ptr(mut self) -> Self {
        self.ptr += 2;
        self.jnzfrm = None;
        self
    }
    fn inv_inc_ptr(mut self) -> Self {
        if let Some(jnzfrm) = self.jnzfrm {
            self.ptr = jnzfrm;
            return self;
        }
        self.ptr -= 2;
        self
    }
    fn adv(mut self) -> ReversibleComputer<impl Iterator<Item = usize>, BI, CI> {
        let a = self
            .combo_operand()
            .map(|combo| 2usize.pow(combo as u32))
            .and_then(move |div| self.a.clone().map(move |a| a / div));
        self.ptr += 2;
        ReversibleComputer {
            a,
            b: self.b,
            c: self.c,
            jnzfrm: self.jnzfrm,
            ptr: self.ptr,
            out: self.out,
            ins: self.ins,
        }
    }
    fn bdv(mut self) -> ReversibleComputer<AI, impl Iterator<Item = usize>, CI> {
        let a = self.a.clone();
        let b = self
            .combo_operand()
            .map(|combo| 2usize.pow(combo as u32))
            .and_then(move |div| self.a.clone().map(move |a| a / div));
        self.ptr += 2;
        ReversibleComputer {
            a,
            b,
            c: self.c,
            jnzfrm: self.jnzfrm,
            ptr: self.ptr,
            out: self.out,
            ins: self.ins,
        }
    }
    fn cdv(mut self) -> ReversibleComputer<AI, BI, impl Iterator<Item = usize>> {
        let a = self.a.clone();
        let c = self
            .combo_operand()
            .map(|combo| 2usize.pow(combo as u32))
            .and_then(move |div| self.a.clone().map(move |a| a / div));
        self.ptr += 2;
        ReversibleComputer {
            a,
            b: self.b,
            c,
            jnzfrm: self.jnzfrm,
            ptr: self.ptr,
            out: self.out,
            ins: self.ins,
        }
    }
    fn bxl(mut self) -> ReversibleComputer<AI, impl Iterator<Item = usize>, CI> {
        let l = self.literal_operand();
        let b = self.b.map(move |b| b ^ l);
        self.ptr += 2;
        ReversibleComputer {
            a: self.a,
            b,
            c: self.c,
            jnzfrm: self.jnzfrm,
            ptr: self.ptr,
            out: self.out,
            ins: self.ins,
        }
    }
    fn bst(mut self) -> ReversibleComputer<AI, impl Iterator<Item = usize>, CI> {
        let b = self.combo_operand().map(|c| c % 8);
        self.ptr += 2;
        ReversibleComputer {
            a: self.a,
            b,
            c: self.c,
            jnzfrm: self.jnzfrm,
            ptr: self.ptr,
            out: self.out,
            ins: self.ins,
        }
    }
    fn bxc(mut self) -> ReversibleComputer<AI, impl Iterator<Item = usize>, CI> {
        let b = self
            .c
            .clone()
            .and_then(move |c| self.b.clone().map(move |b| b ^ c));
        self.ptr += 2;
        ReversibleComputer {
            a: self.a,
            b,
            c: self.c,
            jnzfrm: self.jnzfrm,
            ptr: self.ptr,
            out: self.out,
            ins: self.ins,
        }
    }
    fn jnz(mut self) -> impl Iterator<Item = ReversibleComputer<AI, BI, CI>> {
        let l = self.literal_operand();
        let mut new = self.clone();
        self.a
            .map(move |a| {
                if a != 0 {
                    new.jnzfrm = Some(new.ptr);
                    new.ptr = l;
                    return new.clone();
                }
                new.ptr += 2;
                new.clone()
            })
            .into_iter()
    }
    fn out(mut self) -> impl Iterator<Item = ReversibleComputer<AI, BI, CI>> {
        let mut new = self.clone();
        self.combo_operand()
            .map(move |combo| {
                new.out.push(combo % 8);
                self.ptr += 2;
                new.clone()
            })
            .into_iter()
    }
    // fn inv_adv(mut self) -> Self {
    //     self.ptr -= 2;
    //     self.a *= 2usize.pow(self.combo_operand() as u32);
    //     self
    // }
    // fn inv_bdv(mut self) -> Self {
    //     self.ptr -= 2;
    //     self.b = self.a * 2usize.pow(self.combo_operand() as u32);
    //     self
    // }
    // fn inv_cdv(mut self) -> Self {
    //     self.ptr -= 2;
    //     self.c = self.a * 2usize.pow(self.combo_operand() as u32);
    //     self
    // }
    // fn inv_bxl(mut self) -> Self {
    //     self.ptr -= 2;
    //     self.b ^= self.literal_operand();
    //     self
    // }
    // fn inv_bst(mut self) -> impl Iterator<Item = Self> {
    //     self.ptr -= 2;
    //     self.b = self.combo_operand() % 8;
    //     (0..=usize::MAX).skip(self.b).step_by(8).map(move |i| {
    //         let mut out = self.clone();
    //         // Set depends on combo operand.
    //         out.b = i;
    //         out
    //     })
    // }
    // fn inv_jnz(mut self) -> Self {
    //     // NOTE: Jump case not visible here - we don't know where we jumped
    //     //from!
    //     self.ptr -= 2;
    //     self
    // }
    // fn inv_bxc(mut self) -> Self {
    //     self.ptr -= 2;
    //     self.b ^= self.c;
    //     self
    // }
    // fn inv_out(mut self) -> impl Iterator<Item = Self> {
    //     self.ptr -= 2;
    //     let prev = self.out.pop().unwrap();
    //     (0..=usize::MAX).skip(prev).step_by(8).map(move |i| {
    //         todo!();
    //         let mut out = self.clone();
    //         out.b = i;
    //         out
    //     })
    // }
    // /// (next, is_finished)
    // fn next(mut self) -> (Self, bool) {
    //     match self.cur() {
    //         None => return (self, true),
    //         Some(0) => self = self.adv(),
    //         Some(1) => self = self.bxl(),
    //         Some(2) => self = self.bst(),
    //         Some(3) => self = self.jnz(),
    //         Some(4) => self = self.bxc(),
    //         Some(5) => self = self.out(),
    //         Some(6) => self = self.bdv(),
    //         Some(7) => self = self.cdv(),
    //         _ => panic!(),
    //     }
    //     (self, false)
    // }
    // fn execute(mut self) -> Vec<usize> {
    //     loop {
    //         let fin;
    //         (self, fin) = self.next();
    //         if fin {
    //             break;
    //         }
    //     }
    //     self.out
    // }
}

impl Computer {
    fn from_str(s: &str) -> Computer {
        let mut lines = s.lines();
        let a = lines
            .next()
            .unwrap()
            .trim_start_matches("Register A: ")
            .parse()
            .unwrap();
        let b = lines
            .next()
            .unwrap()
            .trim_start_matches("Register B: ")
            .parse()
            .unwrap();
        let c = lines
            .next()
            .unwrap()
            .trim_start_matches("Register C: ")
            .parse()
            .unwrap();
        let ins = lines
            .nth(1)
            .unwrap()
            .trim_start_matches("Program: ")
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        Computer {
            a,
            b,
            c,
            ptr: 0,
            out: Vec::new(),
            ins,
        }
    }
    fn literal_operand(&self) -> usize {
        self.ins[self.ptr + 1]
    }
    fn combo_operand(&self) -> usize {
        let combo = self.ins[self.ptr + 1];
        match combo {
            0..=3 => combo,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unreachable!(),
            _ => panic!(),
        }
    }
    fn cur(&self) -> Option<usize> {
        self.ins.get(self.ptr).copied()
    }
    fn adv(mut self) -> Self {
        self.a /= 2usize.pow(self.combo_operand() as u32);
        self.ptr += 2;
        self
    }
    fn bxl(mut self) -> Self {
        self.b ^= self.literal_operand();
        self.ptr += 2;
        self
    }
    fn bst(mut self) -> Self {
        self.b = self.combo_operand() % 8;
        self.ptr += 2;
        self
    }
    fn jnz(mut self) -> Self {
        if self.a != 0 {
            self.ptr = self.literal_operand();
            return self;
        }
        self.ptr += 2;
        self
    }
    fn bxc(mut self) -> Self {
        self.b ^= self.c;
        self.ptr += 2;
        self
    }
    fn out(mut self) -> Self {
        self.out.push(self.combo_operand() % 8);
        self.ptr += 2;
        self
    }
    fn bdv(mut self) -> Self {
        self.b = self.a / 2usize.pow(self.combo_operand() as u32);
        self.ptr += 2;
        self
    }
    fn cdv(mut self) -> Self {
        self.c = self.a / 2usize.pow(self.combo_operand() as u32);
        self.ptr += 2;
        self
    }
    fn inv_adv(mut self) -> Self {
        self.ptr -= 2;
        self.a *= 2usize.pow(self.combo_operand() as u32);
        self
    }
    fn inv_bdv(mut self) -> Self {
        self.ptr -= 2;
        self.b = self.a * 2usize.pow(self.combo_operand() as u32);
        self
    }
    fn inv_cdv(mut self) -> Self {
        self.ptr -= 2;
        self.c = self.a * 2usize.pow(self.combo_operand() as u32);
        self
    }
    fn inv_bxl(mut self) -> Self {
        self.ptr -= 2;
        self.b ^= self.literal_operand();
        self
    }
    fn inv_bst(mut self) -> impl Iterator<Item = Self> {
        self.ptr -= 2;
        self.b = self.combo_operand() % 8;
        (0..=usize::MAX).skip(self.b).step_by(8).map(move |i| {
            let mut out = self.clone();
            // Set depends on combo operand.
            out.b = i;
            out
        })
    }
    fn inv_jnz(mut self) -> Self {
        // NOTE: Jump case not visible here - we don't know where we jumped from!
        self.ptr -= 2;
        self
    }
    fn inv_bxc(mut self) -> Self {
        self.ptr -= 2;
        self.b ^= self.c;
        self
    }
    fn inv_out(mut self) -> impl Iterator<Item = Self> {
        self.ptr -= 2;
        let prev = self.out.pop().unwrap();
        (0..=usize::MAX).skip(prev).step_by(8).map(move |i| {
            todo!();
            let mut out = self.clone();
            out.b = i;
            out
        })
    }
    /// (next, is_finished)
    fn next(mut self) -> (Self, bool) {
        match self.cur() {
            None => return (self, true),
            Some(0) => self = self.adv(),
            Some(1) => self = self.bxl(),
            Some(2) => self = self.bst(),
            Some(3) => self = self.jnz(),
            Some(4) => self = self.bxc(),
            Some(5) => self = self.out(),
            Some(6) => self = self.bdv(),
            Some(7) => self = self.cdv(),
            _ => panic!(),
        }
        (self, false)
    }
    // Specialized based on hand compiled input program.
    fn simplified_next(mut self) -> (Self, bool) {
        self.b = (self.a % 8) ^ 1;
        self.c = self.a / 2usize.pow(self.b as u32);
        self.b = self.b ^ 5 ^ self.c;
        self.a /= 8;
        self.out.push(self.b % 8);
        if self.a == 0 {
            return (self, true);
        }
        (self, false)
    }
    // Specialized based on hand compiled input program.
    fn simplified_inv(mut self) -> impl Iterator<Item = Self> {
        let a = (0..=64).map(move |i| i + self.a * 8);
        // c is not relevant. It is overridden by a / 2.pow(b) on every iteration.
        let c = 0;
        // b is not relevant. It is overridden by a % 8 on every iteration.
        let b = 0;
        a.map(move |a| {
            let mut item = self.clone();
            item.a = a;
            item
        })
    }
    fn execute(mut self) -> Vec<usize> {
        loop {
            let fin;
            (self, fin) = self.next();
            if fin {
                break;
            }
        }
        self.out
    }
}

// Bonus function - for debugging.
fn print_instruction(inst: usize, op: usize) -> String {
    match inst {
        0 => format!("adv {}", print_combo(op)),
        1 => format!("bxl {op}"),
        2 => format!("bst {}", print_combo(op)),
        3 => format!("jnz {op}"),
        4 => "bxc".to_string(),
        5 => format!("out {}", print_combo(op)),
        6 => format!("bdv {}", print_combo(op)),
        7 => format!("cdv {}", print_combo(op)),
        _ => unreachable!(),
    }
}

// Bonus function - for debugging.
fn print_combo(op: usize) -> char {
    match op {
        0..4 => char::from_digit(op as u32, 10).unwrap(),
        4 => 'A',
        5 => 'B',
        6 => 'B',
        _ => unreachable!(),
    }
}

// Bonus function - for debugging.
fn print_program(inst: &[usize]) {
    for (inst, op) in inst
        .iter()
        .zip(inst.iter().chain([7].iter()).skip(1))
        .step_by(2)
    {
        println!("{}", print_instruction(*inst, *op));
    }
}

fn get_output(s: &str) -> Vec<usize> {
    Computer::from_str(s).execute()
}

fn smallest_quine(s: &str) -> usize {
    let mut state = Computer::from_str(s);
    // Based on hand compiled values, this is the final state.
    state.a = 0;
    let target_out = state.ins.clone();
    'outer: loop {
        let possible_states = state.clone().simplified_inv();
        let mut found = false;
        for maybe_inv in possible_states {
            let maybe_inv_output = &maybe_inv.clone().execute();
            println!("trying {:?}, {}", maybe_inv_output, maybe_inv.a);
            if &target_out == maybe_inv_output {
                println!("Got it");
                found = true;
                state = maybe_inv;
                break 'outer;
            }
            if target_out.ends_with(maybe_inv_output) {
                println!("Got it");
                found = true;
                state = maybe_inv;
                break;
            }
        }
        if !found {
            panic!("Each inverse should have a result");
        }
    }
    state.a
}

pub(crate) fn part_1(input: String) {
    println!("Program output: {:?}", get_output(&input));
}

pub(crate) fn part_2(input: String) {
    println!("Smallest quine created when a: {}", smallest_quine(&input));
}

#[cfg(test)]
mod tests {
    use super::get_output;
    use crate::day_17::{smallest_quine, Computer};

    const TEST_DATA: &str = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    #[test]
    fn test_part_1() {
        assert_eq!(get_output(TEST_DATA), vec![0, 3, 5, 4, 3, 0]);
    }
    #[test]
    #[ignore = "Ignored due to running time"]
    fn test_part_2() {
        assert_eq!(smallest_quine(TEST_DATA), 117440);
    }
    // #[test]
    // fn test_adv() {
    //     let mut state = Computer::from_str(TEST_DATA);
    //     assert_eq!(state, state.clone().adv().inv_adv());
    // }
    // #[test]
    // fn test_bdv() {
    //     let mut state = Computer::from_str(TEST_DATA);
    //     assert_eq!(state, state.clone().bdv().inv_bdv());
    // }
    // #[test]
    // fn test_cdv() {
    //     let mut state = Computer::from_str(TEST_DATA);
    //     assert_eq!(state, state.clone().cdv().inv_cdv());
    // }
    // #[test]
    // fn test_bxl() {
    //     let mut state = Computer::from_str(TEST_DATA);
    //     assert_eq!(state, state.clone().bxl().inv_bxl());
    // }
}
