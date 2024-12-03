use crate::test_cases;
use crate::common::*;

pub const DAY17_EXAMPLE: &str =
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

pub struct Day17 {
    
}
#[derive(Debug, Clone)]
struct Cave {
    grid: Grid<u8>,
}
struct Crucible {
    x: usize,
    y: usize,
    dir: Direction,
    dir_count: usize,
}

struct SearchHead {
    crucible: Crucible,
    loss: usize,
}

impl Day<17> for Day17 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day17.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let lines = input.lines();
        match part {
            Part::One => {
                lines.count()
            },
            Part::Two => {
                0
            }
        }
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (DAY17_EXAMPLE, 102),
                // (self.input(), 0),
            ],
            test_cases![
                // (DAY17_EXAMPLE, 0),
                // (self.input(), 0),
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
}