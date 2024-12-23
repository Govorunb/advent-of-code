use std::str::FromStr;
use itertools::Either;
use num::Integer;
use crate::*;

aoc_day!(
    day = 10,
    output = usize,
    examples = [
"0123
1234
8765
9876",
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 1),
            (Self::EXAMPLES[1], 36),
            (Self::INPUT, 629),
        ],
        test_cases![
            (Self::EXAMPLES[1], 81),
            (Self::INPUT, 1242),
        ]
    ],
    solve = |input, part| {
        let grid = Grid::from_digits(input, 10);
        // println!("{grid}");
        
        let trailheads = grid.cells()
            .filter_map(|(pt, &cell)| (cell == 0).then_some(pt))
            .collect_vec();
        
        match part {
            Part::One => {
                trailheads.iter()
                    .map(|&pt| Self::trailhead_score(&grid, pt))
                    .sum()
            },
            Part::Two => {
                trailheads.iter()
                    .map(|&pt| Self::trailhead_rating(&grid, pt))
                    .sum()
            }
        }
    }
);

impl Day10 {
    fn trailhead_score(grid: &Grid<usize>, start: Vector2) -> usize {
        let mut seen_nines = FxHashSet::default();
        
        Self::count_nines(grid, start, &mut seen_nines);
        
        seen_nines.len()
    }
    
    fn count_nines(grid: &Grid<usize>, pos: Vector2, seen: &mut FxHashSet<Vector2>) {
        let self_height = grid[pos];
        if self_height == 9 {
            seen.insert(pos);
            return;
        }
        for p in pos.adjacent() {
            let Some(&height) = grid.get(&p) else {continue};
            if height != self_height+1 {continue};
            Self::count_nines(grid, p, seen);
        }
    }
    
    fn trailhead_rating(grid: &Grid<usize>, start: Vector2) -> usize {
        let mut distinct_trails = 0;

        Self::count_trails(grid, start, &mut distinct_trails);

        distinct_trails
    }

    fn count_trails(grid: &Grid<usize>, pos: Vector2, count: &mut usize) {
        let self_height = grid[pos];
        if self_height == 9 {
            *count += 1;
            return;
        }
        for p in pos.adjacent() {
            let Some(&height) = grid.get(&p) else {continue};
            if height != self_height+1 {continue};
            Self::count_trails(grid, p, count);
        }
    }
}
