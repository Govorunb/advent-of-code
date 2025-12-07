use std::{collections::hash_map::Entry, iter};

use itertools::Either;

use crate::*;

aoc_day!(
    day = 7,
    output = usize,
    examples = [
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 21),
            (Self::INPUT, 1656),
        ],
        test_cases![
            (Self::EXAMPLES[0], 40),
            (Self::INPUT, 76624086587804),
        ]
    ],
    solve = |input, part| {
        let _lines = input.lines();
        let mut grid: Grid<char> = input.parse().unwrap();
        let start = grid.find(&'S').unwrap() + Vector2::DOWN;
        match part {
            Part::One => {
                beam_p1(&mut grid, start);
                let hit = grid.cells()
                    .filter(|&(_, c)| c == &'^')
                    .map(|(p, _)| p)
                    .filter(|&p| matches!(grid.get(&(p + Vector2::UP)), Some('|')))
                    .collect_vec();
                for h in &hit {
                    grid[h] = 'v';
                }
                // println!("{grid}");
                hit.len()
            },
            Part::Two => {
                let mut memo = FxHashMap::default();
                beam_p2(&grid, start, &mut memo) + 1
            }
        }
    }
);

fn beam_p1(grid: &mut Grid<char>, start: Vector2) {
    if !grid.bounds().contains(&start) {
        return;
    }
    for p in start.ray(Vector2::DOWN) {
        let Some(c) = grid.get_mut(&p)
            else {break};
        match c {
            '.' => *c = '|',
            '|' => return,
            '^' => {
                beam_p1(grid, p + Vector2::LEFT);
                beam_p1(grid, p + Vector2::RIGHT);
                return;
            },
            _ => {
                println!("erm {c}");
                return;
            }
        }
    }
}

fn beam_p2(grid: &Grid<char>, start: Vector2, memo: &mut FxHashMap<Vector2, usize>) -> usize {
    if !grid.bounds().contains(&start) {
        return 0;
    }
    for (p, &c) in grid.ray(start, Vector2::DOWN) {
        if c == '^' {
            // println!("hit at {p}");
            if let Some(&e) = memo.get(&p) {
                return e;
            }
            let total = 1usize // why? it splits in two
                + beam_p2(grid, p + Vector2::LEFT, memo)
                + beam_p2(grid, p + Vector2::RIGHT, memo);
            memo.insert(p, total);
            return total;
        }
    }
    return 0;
}
