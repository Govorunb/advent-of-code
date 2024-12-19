use std::collections::HashSet;

use crate::*;

aoc_day!(
    day = 3,
    output = usize,
    examples = [""],
    tests = [
        test_cases![
            (">", 2),
            ("^>v<", 4),
            ("^v^v^v^v^v", 2),
            (Self::INPUT, 2081),
        ],
        test_cases![
            ("^v", 3),
            ("^>v<", 3),
            ("^v^v^v^v^v", 11),
            (Self::INPUT, 2341),
        ]
    ],
    solve = |input, part| {
        let heads = part as usize;
        
        let mut grid = Walkers::new(heads);
        for (i, c) in input.chars().enumerate() {
            grid.move_head(i % heads, c.into());
        }
        grid.walked.len()
    }
);

struct Walkers {
    heads: Vec<Vector2>,
    walked: HashSet<Vector2>,
}

impl Walkers {
    fn new(heads: usize) -> Self {
        let starting_pos = Vector2::ZERO;
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
