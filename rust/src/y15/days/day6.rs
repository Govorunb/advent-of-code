use std::ops::Add;
use std::sync::LazyLock;
use crate::*;

aoc_day!(
    day = 6,
    output = usize,
    examples = [
"turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500",
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 998996),
            (Self::INPUT, 377891),
        ],
        test_cases![
            (Self::EXAMPLES[0], 1001996),
            (Self::INPUT, 14110788),
        ]
    ],
    solve = |input, part| {
        const SIDE: usize = 1000;
        let lines = input.lines();
        let instructions = lines
            .map_into::<Instruction>()
            .collect_vec();
        match part {
            Part::One => {
                let mut grid: Grid<bool> = Grid::from_origin(Size::square(SIDE)).unwrap();
                for instruction in instructions.iter() {
                    instruction.apply_p1(&mut grid);
                }
                grid.elements()
                    .filter(|&&l| l)
                    .count()
            },
            Part::Two => {
                let mut grid: Grid<usize> = Grid::from_origin(Size::square(SIDE)).unwrap();
                for instruction in instructions.iter() {
                    instruction.apply_p2(&mut grid);
                }
                grid.elements()
                    .sum()
            }
        }
    }
);

struct Instruction {
    itype: InstructionType,
    rect: Rect,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum InstructionType { On, Off, Toggle }

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