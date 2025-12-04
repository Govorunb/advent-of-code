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
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let mut grid = input.parse::<Grid<char>>().unwrap();
        match part {
            Part::One => {
                grid.cells()
                    .filter(|&(_, c)| *c == '@')
                    .filter(|(pos, _)| pos.around()
                        .filter(|p| matches!(grid.get(&p), Some('@')))
                        .count() < 4)
                    .count()
            },
            Part::Two => {
                let mut removed = 0;
                loop {
                    let removable = grid.cells()
                        .filter(|&(_, c)| *c == '@')
                        .filter(|(pos, _)| pos.around()
                            .filter(|p| matches!(grid.get(&p), Some('@')))
                            .count() < 4)
                        .collect_vec();
                    if removable.len() == 0 {
                        break;
                    }
                    removed += removable.len();
                    let mut grid2 = grid.clone();
                    for r in removable {
                        grid2[r.0] = '.';
                    }
                    grid = grid2;
                }
                removed
            }
        }
    }
);
