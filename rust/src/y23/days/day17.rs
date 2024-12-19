use crate::*;

aoc_day!(
    day = 17,
    output = usize,
    examples = [
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 102),
            // (Self::INPUT, 0),
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let lines = input.lines();
        match part {
            Part::One => {
                lines.count()
            },
            Part::Two => {
                0
            }
        }
    }
);

#[derive(Debug, Clone)]
struct Cave {
    grid: Grid<u8>,
}
struct Crucible {
    coords: Vector2,
    dir: Direction,
    dir_count: usize,
}

struct SearchHead {
    crucible: Crucible,
    loss: usize,
}
