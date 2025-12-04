use crate::*;

aoc_day!(
    day = 4,
    output = usize,
    examples = [
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 13),
            (Self::INPUT, 1344),
        ],
        test_cases![
            (Self::EXAMPLES[0], 43),
            (Self::INPUT, 8112),
        ]
    ],
    solve = |input, part| {
        let mut grid: Grid<_> = input.parse().unwrap();
        match part {
            Part::One => {
                grid.cells()
                    .filter(|&(pos, c)| c == &'@' && accessible(&pos, &grid))
                    .count()
            },
            Part::Two => {
                let mut removed = 0;
                let mut removable = vec![];
                let mut next_check = FxHashSet::default();
                loop {
                    debug_assert!(removable.is_empty());
                    // removing a roll only affects rolls that were around
                    // therefore, we don't have to check the full grid
                    if next_check.len() == 0 {
                        removable.extend(grid.bounds().into_iter()
                            .filter(|pos| grid[pos] == '@' && accessible(&pos, &grid))
                        );
                    } else {
                        removable.extend(next_check.drain()
                            .filter(|pos| accessible(&pos, &grid))
                        );
                    }
                    if removable.len() == 0 {
                        break;
                    }
                    removed += removable.len();
                    for p in &removable {
                        grid[p] = '.';
                    }
                    
                    debug_assert!(next_check.is_empty());
                    next_check.extend(removable.drain(..)
                        .flat_map(|p| p.around())
                        .filter(|p| matches!(grid.get(&p), Some('@')))
                    );
                }
                removed
            }
        }
    }
);

fn accessible(pos: &Vector2, grid: &Grid<char>) -> bool {
    pos.around()
        .filter(|p| matches!(grid.get(p), Some('@')))
        // you might think: count() will iterate up to all 8, we only care about 4
        // but that case seems to already be optimized (at the very least, .nth(3).is_none() and .skip(3).next().is_none() are much slower)
        .count() < 4
}