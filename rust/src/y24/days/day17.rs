use std::sync::atomic::AtomicBool;
use crate::*;

pub struct Day17 {
    
}

#[derive(Debug, Clone)]
struct Cpu {
    registers: [usize; 3],
    ip: usize,
    program: Vec<usize>,
    debug: bool,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
}

impl From<usize> for Instruction {
    fn from(value: usize) -> Self {
        match value {
            0 => Instruction::adv,
            1 => Instruction::bxl,
            2 => Instruction::bst,
            3 => Instruction::jnz,
            4 => Instruction::bxc,
            5 => Instruction::out,
            6 => Instruction::bdv,
            7 => Instruction::cdv,
            _ => unreachable!()
        }
    }
}
impl Day<17> for Day17 {
    type Output = String;
    const INPUT: &'static str = include_str!("../Input/day17.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let regex = Regex::new(r#"Register A: (?<A>-?\d+)
Register B: (?<B>-?\d+)
Register C: (?<C>-?\d+)

Program: (?<prog>.*)"#).unwrap();
        let mut cpu = regex.captures_iter(input)
            .map(|c| {
                let program = c.str("prog").chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                    .collect_vec();
                Cpu {
                    registers: ["A","B","C"].map(|name| c.usize(name)),
                    ip: 0,
                    program,
                    debug: false
                }
            })
            .nth(0).unwrap();
        match part {
            Part::One => {
                cpu.run()
                    .map(|num| num.to_string())
                    .join(",")
            },
            Part::Two => {
                // cpu.debug = true;
                let program = cpu.program.clone();
                Self::p2_reverse(&mut cpu, &program, program.len()-1, 0)
                    .unwrap()
                    .to_string()
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], "4,6,3,5,6,3,5,2,1,0".to_string()),
                (Self::INPUT, "1,7,2,1,4,1,5,4,0".to_string()),
            ],
            test_cases![
                (Self::EXAMPLES[1], "117440".to_string()),
                (Self::INPUT, "37221261688308".to_string()),
            ]
        ]
    }

}

impl Default for Day17 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day17 {
    pub fn new() -> Self {
        Self {
        }
    }
    // unique to programs with a single `adv 3` then a single `jmp 0`
    // works on my ~~machine~~ input 👍
    fn p2_reverse(cpu: &mut Cpu, program: &[usize], i: usize, curr: usize) -> Option<usize> {
        // println!("iteration {i} at {curr}");
        for last_bits in 0..8 {
            let a = curr << 3 | last_bits;
            
            // println!("[{i}] trying {last_bits:b}... loading [{a}, 0, 0]");
            cpu.registers = [a,0,0];
            cpu.ip = 0;
            let out = cpu.run().next().unwrap();
            
            if program[i] != out {continue}
            
            if i == 0 {
                // println!("Success!");
                return Some(a)
            }

            if let Some(partial) = Self::p2_reverse(cpu, program, i-1, a) {
                return Some(partial)
            }
        }
        None
    }
}

#[derive(Debug)]
enum Combo {
    Literal(usize),
    Register(usize)
}

impl Cpu {
    fn run(&mut self) -> impl Iterator<Item = usize> {
        std::iter::from_fn(|| {
            loop {
                let res = self.step();
                if !res.0 {return None}
                if res.1.is_some() {
                    return res.1;
                }
            }
        })
    }
    fn step(&mut self) -> (bool, Option<usize>) {
        let Some((i, o)) = self.fetch()
            else {return (false, None)};

        (true, self.execute(i,o))
    }
    fn fetch(&mut self) -> Option<(Instruction, usize)> {
        let &instr = self.program.get(self.ip)?;
        let &oper = self.program.get(self.ip+1)?;
        self.ip += 2;
        
        Some((instr.into(), oper))
    }
    fn execute(&mut self, i: Instruction, o: usize) -> Option<usize> {
        if self.debug {
            println!("executing {i:?} {o} with {:?}", self.registers);
        }
        match i {
            Instruction::adv => self.shift(o, 0),
            Instruction::bxl => {
                if cfg!(debug_assertions) && self.debug {
                    println!("B ^= {o}");
                }
                self.registers[1] ^= o
            },
            Instruction::bst => {
                // x % 8 is equivalent to x & 0b0111
                if cfg!(debug_assertions) && self.debug {
                    println!("B = {} & 7", Self::combo_repr(o));
                }
                self.registers[1] = self.combo(o) & 7
            },
            Instruction::jnz => if self.registers[0] != 0 { self.ip = o },
            Instruction::bxc => self.registers[1] ^= self.registers[2],
            Instruction::out => {
                let res = self.combo(o) & 7;
                if cfg!(debug_assertions) && self.debug {
                    println!("output {} & 7 = {}", Self::combo_repr(o), res);
                }
                return Some(res)
            },
            Instruction::bdv => self.shift(o, 1),
            Instruction::cdv => self.shift(o, 2),
        };
        None
    }
    
    fn shift(&mut self, o: usize, dest: usize) {
        // x / 2^y is equivalent to x >> y
        let result = self.registers[0] >> self.combo(o);
        if cfg!(debug_assertions) && self.debug {
            println!("A << {} = {result} -> {}", Self::combo_repr(o), Self::reg(dest));
        }
        self.registers[dest] = result;
    }
    
    fn combo(&self, op: usize) -> usize {
        match Self::combo_addr(op) {
            Combo::Literal(val) => val,
            Combo::Register(reg) => self.registers[reg],
        }
    }
    
    fn combo_addr(op: usize) -> Combo {
        match op {
            0..=3 => Combo::Literal(op),
            4..=6 => Combo::Register(op-4),
            7 => unreachable!("reserved"),
            _ => unreachable!()
        }
    }
    
    fn combo_repr(op: usize) -> char {
        match Self::combo_addr(op) {
            Combo::Literal(l) => char::from_digit(l as u32, 10).unwrap(),
            Combo::Register(r) => Self::reg(r),
        }
    }
    
    fn reg(reg: usize) -> char {
        ['A','B','C'][reg]
    }
}