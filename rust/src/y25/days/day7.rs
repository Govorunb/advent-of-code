use std::{collections::hash_map::Entry, iter};

use itertools::Either;

use crate::*;

aoc_day!(
    day = 7,
    output = usize,
    examples = [
"\
.......S.......
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
        let mut grid: Grid<u8> = input.parse().unwrap();
        let start = grid.find(&b'S').unwrap() + Vector2::DOWN;
        match part {
            Part::One => {
                beam_p1(&mut grid, start);
                grid.cells()
                    .filter_map(|(p, &c)| (c == b'^').then_some(p))
                    .filter(|&p| matches!(grid.get(&(p + Vector2::UP)), Some(b'|')))
                    .count()
            },
            Part::Two => {
                let mut memo = FxHashMap::default();
                beam_p2(&grid, start, &mut memo) + 1
            }
        }
    }
);

fn beam_p1(grid: &mut Grid<u8>, start: Vector2) {
    for p in start.ray(Vector2::DOWN) {
        let Some(c) = grid.get_mut(&p)
            else {break};
        match c {
            b'|' => return,
            b'.' => *c = b'|',
            b'^' => {
                beam_p1(grid, p + Vector2::LEFT);
                beam_p1(grid, p + Vector2::RIGHT);
                return;
            },
            _ => unreachable!(),
        }
    }
}

fn beam_p2(grid: &Grid<u8>, start: Vector2, memo: &mut FxHashMap<Vector2, usize>) -> usize {
    let mut ray = grid.ray(start, Vector2::DOWN);
    let Some(p) = ray.find_map(|(p, &c)| (c == b'^').then_some(p))
        else {return 0};
    
    if let Some(&e) = memo.get(&p) {
        return e;
    }
    // hello again y23d16
    let total = 1usize
        + beam_p2(grid, p + Vector2::LEFT, memo)
        + beam_p2(grid, p + Vector2::RIGHT, memo);
    memo.insert(p, total);
    return total;
}
