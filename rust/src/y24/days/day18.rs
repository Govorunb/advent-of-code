use crate::*;

pub struct Day18 {
    
}

impl Day<18> for Day18 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day18.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let bytes = input.lines()
            .map(|line| {
                let (x,y) = line.split_once(',').unwrap();
                Vector2 {
                    x: x.parse::<isize>().unwrap(),
                    y: y.parse::<isize>().unwrap()
                }
            }).collect_vec();
        let is_example = input.lines().count() < 100;
        let side = if is_example {6} else {70};
        let size = Size {width: side, height: side};
        let mut grid: Grid<bool> = Grid::from_origin(size).unwrap();
        match part {
            Part::One => {
                let fall_amt = if is_example {12} else {1024};
                for byte in &bytes[..fall_amt] {
                    grid[byte] = true;
                }
                
                0
            },
            Part::Two => {
                0
            }
        }
    }
    const EXAMPLES: &'static [&'static str] = &[
"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
    ];
    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (Self::EXAMPLES[0], 22),
                // (Self::INPUT, 0),
            ],
            test_cases![
                // (Self::EXAMPLES[0], 0),
                // (Self::INPUT, 0),
            ]
        ]
    }
}

impl Default for Day18 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day18 {
    pub fn new() -> Self {
        Self {
        }
    }
}