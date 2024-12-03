use std::collections::HashSet;

use crate::test_cases;
use crate::common::*;

pub struct Day3 {
}

struct Grid {
    heads: Vec<Point>,
    walked: HashSet<Point>,
}

impl Grid {
    fn new(heads: usize) -> Self {
        let starting_pos = &Point { x: 0, y: 0 };
        Self {
            heads: Vec::from_iter(std::iter::repeat(starting_pos).cloned().take(heads)),
            walked: HashSet::from([*starting_pos]),
        }
    }

    pub fn move_head(&mut self, head: usize, dir: Direction) {
        let head_ref = self.heads.get_mut(head).unwrap();
        match dir {
            Direction::Up => {
                head_ref.y -= 1;
            }
            Direction::Down => {
                head_ref.y += 1;
            }
            Direction::Left => {
                head_ref.x -= 1;
            }
            Direction::Right => {
                head_ref.x += 1;
            }
        }
        self.walked.insert(*head_ref);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Day<3> for Day3 {
    type Output = usize;
    const INPUT: &'static str = include_str!("../Input/day3.txt");
    fn solve_part(&self, input: &str, part: Part) -> Self::Output {
        match part {
            Part::One => {
                let mut grid = Grid::new(1);
                for dir in input.chars().map(parse_direction) {
                    grid.move_head(0, dir);
                }
                grid.walked.len()
            },
            Part::Two => {
                let mut grid = Grid::new(2);
                for (i, c) in input.char_indices() {
                    let dir = parse_direction(c);
                    let head_to_move = i % 2;
                    grid.move_head(head_to_move, dir);
                }
                grid.walked.len()
            }
        }
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
        '>' => Direction::Right,
        '<' => Direction::Left,
        '^' => Direction::Up,
        'v' => Direction::Down,
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