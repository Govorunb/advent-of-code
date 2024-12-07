use std::collections::HashSet;

use crate::test_cases;
use crate::common::*;

pub struct Day3 {
}

struct Walkers {
    heads: Vec<Vector2>,
    walked: HashSet<Vector2>,
}

impl Walkers {
    fn new(heads: usize) -> Self {
        let starting_pos = Vector2 { x: 0, y: 0 };
        Self {
            heads: vec![starting_pos; heads],
            walked: HashSet::from([starting_pos]),
        }
    }

    pub fn move_head(&mut self, head: usize, dir: Direction) {
        let head_ref = self.heads.get_mut(head).unwrap();
        *head_ref += dir;
        self.walked.insert(*head_ref);
    }
}

impl Day<3> for Day3 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day3.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        let heads = match part {
            Part::One => 1,
            Part::Two => 2,
        };
        let mut grid = Walkers::new(heads);
        for (i, c) in input.chars().enumerate() {
            let dir = parse_direction(c);
            grid.move_head(i % heads, dir);
        }
        grid.walked.len()
    }

    fn test_cases(&self) -> [Vec<Self::TestCase>; 2] {
        [
            test_cases![
                (">", 2),
                ("^>v<", 4),
                ("^v^v^v^v^v", 2),
            ],
            test_cases![
                ("^v", 3),
                ("^>v<", 3),
                ("^v^v^v^v^v", 11),
            ]
        ]
    }
}

fn parse_direction(c: char) -> Direction {
    match c {
        '>' => Direction::East,
        '<' => Direction::West,
        '^' => Direction::North,
        'v' => Direction::South,
        _ => unreachable!()
    }
}

impl Default for Day3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day3 {
    pub fn new() -> Self {
        Self {
        }
    }
}