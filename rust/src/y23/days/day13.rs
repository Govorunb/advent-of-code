#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::{Either, Itertools};
use rustc_hash::FxHashMap;
use std::{collections::HashSet, hash::Hasher};
use crate::*;

aoc_day!(
    day = 13,
    output = usize,
    examples = [
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 5),
            (Self::EXAMPLES[1], 400),
            (Self::INPUT, 35360),
        ],
        test_cases![
            (Self::EXAMPLES[0], 300),
            (Self::EXAMPLES[1], 100),
            (Self::INPUT, 36755),
        ]
    ],
    solve = |input, part| {
        let valleys = input.split("\n\n")
            .map(Valley::parse).collect_vec();
        let hashes = FxHashMap::from_iter(valleys.iter()
            .map(|v| (v.grid.clone(), v.make_hashes())));
        let reflections = valleys.iter()
            .map(|g| g.find_reflection(None, &hashes))
            .collect_vec();
        match part {
            Part::One => {
                Either::Left(reflections.into_iter())
            },
            Part::Two => {
                Either::Right(
                    valleys.iter()
                        .zip_eq(reflections)
                        .map(|(g, r)| g.find_reflection(Some(r), &hashes))
                )
            }
        }
        .map(|r| r.score())
        .sum()
    }
);


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
enum Cell {
    Ash,
    Rock,
}

enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]

struct Valley {
    grid: Grid<Cell>,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Ash,
            '#' => Cell::Rock,
            _ => unreachable!()
        }
    }
}

impl Valley {
    fn parse(input: &str) -> Self {
        let grid = input.parse()
            .expect("failed to parse grid");
        Self {
            grid,
        }
    }

    fn make_hashes(&self) -> (Vec<u64>, Vec<u64>) {
        (self.make_hashes_v(), self.make_hashes_h())
    }

    fn make_hashes_v(&self) -> Vec<u64> {
        self.grid.cols()
            .map(|col| col.iter())
            .map(Self::map_hashes)
            .collect_vec()
    }

    fn make_hashes_h(&self) -> Vec<u64> {
        self.grid.rows()
        .map(|row| row.iter())
        .map(Self::map_hashes)
        .collect_vec()
    }

    fn map_hashes<'a>(cells: impl Iterator<Item = &'a Cell>) -> u64 {
        cells
            .enumerate()
            .filter_map(|(i, c)| if let Cell::Rock = c {Some(i)} else {None})
            .fold(0, |acc, i| acc | (1 << i))
    }

    fn find_reflection(&self, previous: Option<Reflection>, hashes: &FxHashMap<Grid<Cell>, (Vec<u64>, Vec<u64>)>) -> Reflection {
        let has_smudge = previous.is_some();
        let tolerance = if has_smudge { 1 } else { 0 };
        let (hashes_v, hashes_h) = hashes.get(&self.grid).expect("call make_hashes first");
        'nextCol: for col in 0..(hashes_v.len()-1) {
            let mut spread = 1;
            let mut tolerance_left = tolerance;
            while col >= (spread-1) && col + spread < hashes_v.len() {
                let left = hashes_v[col - (spread-1)];
                let right = hashes_v[col + spread];
                let differing_bytes = (left ^ right).count_ones();
                if differing_bytes == 0 {}
                else if differing_bytes <= tolerance_left {
                    tolerance_left -= 1;
                } else {
                    continue 'nextCol;
                }
                spread += 1;
            }
            
            if let Some(Reflection::Vertical(prev_col)) = previous {
                if prev_col == col+1 { continue }
            }
            return Reflection::Vertical(col+1);
        }
        'outer: for row in 0..(hashes_h.len()-1) {
            let mut spread = 1;
            let mut tolerance_left = tolerance;
            while row >= (spread-1) && row + spread < hashes_h.len() {
                let left = hashes_h[row - (spread-1)];
                let right = hashes_h[row + spread];
                let differing_bytes = (left ^ right).count_ones();
                if differing_bytes == 0 {}
                else if differing_bytes <= tolerance_left {
                    tolerance_left -= 1;
                } else {
                    continue 'outer;
                }
                spread += 1;
            }

            if let Some(Reflection::Horizontal(prev_row)) = previous {
                if prev_row == row+1 { continue }
            }
            return Reflection::Horizontal(row+1);
        }
        // just a few meaningless statements to pad out the asm so i can break here
        let mut x = 5;
        x += 10;
        println!("{}", x);
        panic!("should have a reflection")
    }
}

impl Reflection {
    fn score(self) -> usize {
        match self {
            Reflection::Vertical(x) => x,
            Reflection::Horizontal(y) => y * 100,
        }
    }
}
