use std::sync::LazyLock;
use crate::*;

pub const DAY7_EXAMPLE: &str =
"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> a"; // replaced 'i' with 'a' to be able to test

pub struct Day7 {}

#[derive(Clone, Eq, Ord, PartialOrd, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
enum Op<'a> {
    STORE(Operand<'a>),
    NOT(Operand<'a>),
    AND(Operand<'a>, Operand<'a>),
    OR(Operand<'a>, Operand<'a>),
    LSHIFT(Operand<'a>, Operand<'a>),
    RSHIFT(Operand<'a>, Operand<'a>),
}

#[derive(Clone, Eq, Ord, PartialOrd, PartialEq)]
enum Operand<'a> {
    Numeric(u16),
    Wire(&'a str),
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Instruction<'a> {
    op: Op<'a>,
    target: &'a str,
}

struct Emulator<'a> {
    wires: FxHashMap<&'a str, Instruction<'a>>,
}

impl<'a> Emulator<'a> {
    pub fn get_operand(&mut self, operand: Operand<'a>) -> u16 {
        match operand {
            Operand::Numeric(num) => num,
            Operand::Wire(wire) => {
                let instruction = self.wires.get(wire).unwrap();
                let val = match instruction.op.clone() {
                    Op::STORE(val) => self.get_operand(val),
                    Op::NOT(operand) => !self.get_operand(operand),
                    Op::AND(lhs, rhs) => self.get_operand(lhs) & self.get_operand(rhs),
                    Op::OR(lhs, rhs) => self.get_operand(lhs) | self.get_operand(rhs),
                    Op::LSHIFT(lhs, rhs) => self.get_operand(lhs) << self.get_operand(rhs),
                    Op::RSHIFT(lhs, rhs) => self.get_operand(lhs) >> self.get_operand(rhs),
                };
                self.wires.insert(wire, Instruction {
                    op: Op::STORE(Operand::Numeric(val)),
                    target: wire,
                });
                val
            }
        }
    }
}

impl<'a> Operand<'a> {
    pub fn from(s: &'a str) -> Self {
        if let Ok(num) = s.parse::<u16>() {
            Operand::Numeric(num)
        } else {
            Operand::Wire(s)
        }
    }
}

impl Day<7> for Day7 {
    type Output = u16;
    const INPUT: &'static str = include_str!("../Input/day7.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        let mut instructions: FxHashMap<&str, Instruction> = lines
            .map(Instruction::from)
            .map(|i| (i.target, i))
            .collect();
        let mut emulator = Emulator { wires: instructions.clone() };
        let a_signal = emulator.get_operand(Operand::Wire("a"));
        match part {
            Part::One => {
                a_signal
            }
            Part::Two => {
                instructions.insert("b", Instruction {
                    op: Op::STORE(Operand::Numeric(a_signal)),
                    target: "b"
                });
                emulator = Emulator { wires: instructions };
                emulator.get_operand(Operand::Wire("a"))
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY7_EXAMPLE, 65079),
                (self.input(), 956),
            ],
            test_cases![
                // (DAY7_EXAMPLE, 0),
                (self.input(), 40149),
            ],
        ]
    }
}

impl Default for Day7 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day7 {
    pub fn new() -> Self {
        Self {}
    }
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?<lhs>(?<store>\\w+)|NOT (?<not>\\w+)|(?<op1>\\w+) (?<op>AND|OR|[LR]SHIFT) (?<op2>\\w+)) -> (?<rhs>\\w+)").unwrap());
impl<'a> From<&'a str> for Instruction<'a> {
    fn from(line: &'a str) -> Self {
        let captures = REGEX.captures(line).unwrap();
        let target = captures.name("rhs").unwrap().as_str();
        let op: Op = if let Some(store) = captures.name("store").map(|m| m.as_str()) {
            Op::STORE(Operand::from(store))
        } else if let Some(not) = captures.name("not").map(|m| m.as_str()) {
            Op::NOT(Operand::from(not))
        } else if let Some(binary) = captures.name("op").map(|m| m.as_str()) {
            let operand1 = Operand::from(captures.name("op1").unwrap().as_str());
            let operand2 = Operand::from(captures.name("op2").unwrap().as_str());
            match binary {
                "AND" => Op::AND(operand1, operand2),
                "OR" => Op::OR(operand1, operand2),
                "LSHIFT" => Op::LSHIFT(operand1, operand2),
                "RSHIFT" => Op::RSHIFT(operand1, operand2),
                _ => unreachable!()
            }
        } else { unreachable!() };
        
        Self { op, target }
    }
}