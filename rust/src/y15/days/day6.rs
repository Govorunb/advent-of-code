use crate::*;
use std::ops::Add;
use std::sync::LazyLock;

pub struct Day6 {
}

struct Instruction {
    itype: InstructionType,
    rect: Rect,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum InstructionType { On, Off, Toggle }

impl Day<6> for Day6 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day6.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        let instructions = lines
            .map_into::<Instruction>()
            .collect_vec();
        match part {
            Part::One => {
                let mut grid: Grid<bool> = Grid::from_origin((1000,1000).into()).unwrap();
                for instruction in instructions.iter() {
                    instruction.apply_p1(&mut grid);
                }
                grid.elements()
                    .filter(|&&l| l)
                    .count()
            },
            Part::Two => {
                let mut grid: Grid<usize> = Grid::from_origin((1000,1000).into()).unwrap();
                for instruction in instructions.iter() {
                    instruction.apply_p2(&mut grid);
                }
                grid.elements()
                    .sum()
            }
        }
    }

    const EXAMPLES: &'static [&'static str] = &[
"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 998996),
                (Self::INPUT, 377891),
            ],
            test_cases![
                (Self::EXAMPLES[0], 1001996),
                (Self::INPUT, 14110788),
            ]
        ]
    }
}

impl Default for Day6 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day6 {
    pub fn new() -> Self {
        Self {}
    }
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?<type>turn on|turn off|toggle) (?<c1x>\\d+),(?<c1y>\\d+) through (?<c2x>\\d+),(?<c2y>\\d+)").unwrap());
impl Instruction {
    pub fn parse(line: &str) -> Self {
        let m = REGEX.captures(line).unwrap();

        let itype = match m.name("type").unwrap().len() {
            6 => InstructionType::Toggle,
            7 => InstructionType::On,
            8 => InstructionType::Off,
            _ => unreachable!()
        };
        let tl = m.vec2("c1x", "c1y");
        let br = m.vec2("c2x", "c2y");
        Instruction {
            itype,
            rect: Rect::from_corners(tl, br).unwrap()
        }
    }
    
    pub fn apply_p1(&self, grid: &mut Grid<bool>) {
        for y in self.rect.y_range() {
            for x in self.rect.x_range() {
                let pt = (x,y).into();
                let cell = grid.get_mut(&pt).unwrap();
                *cell = match self.itype {
                    InstructionType::On => true,
                    InstructionType::Off => false,
                    InstructionType::Toggle => !*cell,
                };
            }
        }
    }
    pub fn apply_p2(&self, grid: &mut Grid<usize>) {
        for y in self.rect.y_range() {
            for x in self.rect.x_range() {
                let pt = (x,y).into();
                let cell = grid.get_mut(&pt).unwrap();
                *cell = match self.itype {
                    InstructionType::On => cell.add(1),
                    InstructionType::Off => cell.saturating_sub(1),
                    InstructionType::Toggle => cell.add(2),
                };
            }
        }
    }
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        Instruction::parse(line)
    }
}