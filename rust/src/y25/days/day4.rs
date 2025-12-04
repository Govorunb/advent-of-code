use std::collections::VecDeque;

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
        let grid: Grid<_> = input.parse().unwrap();
        // insert 3 paragraph rant about how "adjacent" actually means the 4 cells in cardinal directions
        // and "around" should instead be used for the 8 with diagonals
        let mut adjacency_grid = grid.map_clone_cells(|(pos, &c)| {
            (c == '@').then(|| pos.around()
                .filter(|p| matches!(grid.get(p), Some('@')))
                .count() as u8
            )
        });
        match part {
            Part::One => {
                adjacency_grid.elements()
                    .filter(|adj| matches!(adj, Some(..4)))
                    .count()
            },
            Part::Two => {
                // removing a roll only affects other rolls nearby
                let mut removed = 0;
                let mut next_check = VecDeque::from_iter(
                    adjacency_grid.cells()
                        .filter_map(|(p, c)| (matches!(c, Some(..4))).then_some(p))
                );
                while let Some(p) = next_check.pop_back() {
                    if adjacency_grid[p].take().is_none() { continue }
                    removed += 1;
                    
                    for a in p.around() {
                        let Some(Some(adj)) = adjacency_grid.get_mut(&a)
                            else {continue};
                        *adj -= 1;
                        if *adj < 4 {
                            next_check.push_back(a);
                        }
                    }
                }
                removed
            }
        }
    }
);
