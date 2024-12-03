#[derive(Clone, Debug)]
struct Instruction {
    kind: InstructionType,
    args: Vec<usize>,
}

#[derive(Clone, Debug)]
enum InstructionType {
    Mul,
    Do,
    Dont,
}

#[derive(Clone)]
enum ParserState {
    Invalid,
    AwaitInstruction(String),
    ParseArgs(InstructionType, Vec<String>),
    Complete(Instruction),
}

enum CalculatorState {
    Do(usize),
    Dont(usize),
}

impl CalculatorState {
    fn to_dont(self) -> Self {
        match self {
            CalculatorState::Do(n) => CalculatorState::Dont(n),
            CalculatorState::Dont(n) => CalculatorState::Dont(n),
        }
    }
    fn to_do(self) -> Self {
        match self {
            CalculatorState::Do(n) => CalculatorState::Do(n),
            CalculatorState::Dont(n) => CalculatorState::Do(n),
        }
    }
    fn val(self) -> usize {
        match self {
            CalculatorState::Do(n) => n,
            CalculatorState::Dont(n) => n,
        }
    }
}

impl Instruction {
    fn try_parse(kind: InstructionType, args: Vec<String>) -> Option<Self> {
        if matches!(kind, InstructionType::Mul) {
            if args.len() != 2 {
                return None;
            }
            let args = args
                .into_iter()
                .map(|s| str::parse(&s).ok())
                .collect::<Option<Vec<_>>>()?;
            return Some(Self { kind, args });
        }
        if !args.is_empty() {
            return None;
        }
        Some(Instruction { kind, args: vec![] })
    }
}

impl InstructionType {
    fn try_parse(s: &str) -> Option<Self> {
        if s.ends_with("mul") {
            return Some(InstructionType::Mul);
        }
        if s.ends_with("do") {
            return Some(InstructionType::Do);
        }
        if s.ends_with("don't") {
            return Some(InstructionType::Dont);
        }
        None
    }
}

fn parse_instructions(s: String) -> Vec<Instruction> {
    let mut output = vec![];
    let mut state = ParserState::Invalid;
    for c in s.chars() {
        state = match state {
            ParserState::Invalid | ParserState::Complete(_) => {
                ParserState::AwaitInstruction(c.into())
            }
            ParserState::AwaitInstruction(tmp) => {
                if c == '(' {
                    if let Some(i) = InstructionType::try_parse(&tmp) {
                        ParserState::ParseArgs(i, vec![])
                    } else {
                        ParserState::Invalid
                    }
                } else {
                    ParserState::AwaitInstruction(format!("{tmp}{c}"))
                }
            }
            ParserState::ParseArgs(i, mut v) => {
                if c == ')' {
                    if let Some(instruction) = Instruction::try_parse(i, v) {
                        ParserState::Complete(instruction)
                    } else {
                        ParserState::Invalid
                    }
                } else if c == ',' {
                    v.push(String::new());
                    ParserState::ParseArgs(i, v)
                } else if v.is_empty() {
                    v.push(c.into());
                    ParserState::ParseArgs(i, v)
                } else if !c.is_ascii_digit() {
                    ParserState::AwaitInstruction(c.to_string())
                } else {
                    let len = v.len();
                    if let Some(arg) = v.get_mut(len - 1) {
                        arg.push(c)
                    }
                    ParserState::ParseArgs(i, v)
                }
            }
        };
        if let ParserState::Complete(ref instruction) = state {
            output.push(instruction.clone());
        }
    }
    output
}

pub fn part_1(file: String) {
    let instructions = parse_instructions(file);
    let output = instructions.into_iter().fold(0, |acc, e| {
        let Instruction { kind, args } = e;
        if matches!(kind, InstructionType::Mul) {
            return acc + args[0] * args[1];
        }
        acc
    });
    println!("{output}");
}
pub fn part_2(file: String) {
    let instructions = parse_instructions(file);
    let mut state = CalculatorState::Do(0);
    for i in instructions {
        match i {
            Instruction {
                kind: InstructionType::Do,
                ..
            } => state = state.to_do(),
            Instruction {
                kind: InstructionType::Dont,
                ..
            } => state = state.to_dont(),
            Instruction {
                kind: InstructionType::Mul,
                args,
            } => {
                if let CalculatorState::Do(n) = state {
                    state = CalculatorState::Do(n + args[0] * args[1])
                }
            }
        }
    }
    let output = state.val();
    println!("{output}");
}
