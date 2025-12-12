use std::str::FromStr;

use crate::*;

aoc_day!(
    day = 12,
    output = usize,
    examples = [
"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
    ],
    tests = [
        test_cases![
            (Self::EXAMPLES[0], 2),
            // (Self::INPUT, 0),
        ],
        test_cases![
            // (Self::EXAMPLES[0], 0),
            // (Self::INPUT, 0),
        ]
    ],
    solve = |input, part| {
        let mut sections = input.split("\n\n").collect_vec();
        let regions_s = sections.pop().unwrap();
        assert_eq!(sections.len(), 6); // oh ok only 6 shapes
        let shapes: Vec<Grid<char>> = sections.iter().map(|&l4| {
            Grid::from_str(&l4[3..]).unwrap() // "5:\n"
        }).collect_vec();
        assert_eq!(shapes[0].bounds(), Rect::from_origin((3,3).into()).unwrap());
        let regions = regions_s.lines().map(|l| {
            let (size_s, counts_s) = l.split_once(": ").unwrap();
            let (w,h) = size_s.split_once('x').unwrap();
            let size: Size = (w,h).map(|c| c.parse::<usize>().unwrap()).into();
            let counts = counts_s.split_whitespace().map(|c| c.parse::<usize>().unwrap()).collect_vec();
            (size, counts)
        }).collect_vec();

        match part {
            Part::One => {
                println!("{shapes:?}\n\t{regions:?}");
                0
            },
            Part::Two => {
                0
            }
        }
    }
);
